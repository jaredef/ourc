/* softfloat.c -- freestanding IEEE-754 binary64/binary32 soft-float.
 *
 * Pure integer implementation on IEEE bit patterns. No libc, no libm, no
 * float/double types in the logic. Round-to-nearest, ties-to-even.
 *
 * Integer ops relied on (all on unsigned long = 64-bit and unsigned = 32-bit):
 *   +  -  *  (64x64->low64), <<  >>  &  |  ^  ~, ==,<,>  and for DIVIDE a
 *   restoring long-division loop (shift+subtract+compare only). The one 128-bit
 *   product needed by __muldf3 is synthesized from 32-bit limb multiplies
 *   (32x32->64), so no __int128 and no 64-bit hardware divide are required.
 *   count-leading-zeros is a portable loop (clz64/clz32), not a builtin.
 *
 * See NOTES.md for the technique writeup.
 */
#include "softfloat.h"

typedef unsigned long  u64;
typedef unsigned int   u32;
typedef long           i64;
typedef int            i32;

/* ===== binary64 field layout ===== */
#define D_MANT_BITS 52
#define D_EXP_BIAS  1023
#define D_EXP_MAX   0x7ff
#define D_SIGN      (0x8000000000000000UL)
#define D_MANT_MASK (0x000fffffffffffffUL)
#define D_QNAN      (0x7ff8000000000000UL)  /* canonical quiet NaN */
#define D_INF       (0x7ff0000000000000UL)

/* ===== binary32 field layout ===== */
#define F_MANT_BITS 23
#define F_EXP_BIAS  127
#define F_EXP_MAX   0xff
#define F_SIGN      (0x80000000U)
#define F_MANT_MASK (0x007fffffU)
#define F_QNAN      (0x7fc00000U)
#define F_INF       (0x7f800000U)

/* count leading zeros, portable loops (value != 0 assumed) */
static int clz64(u64 x) {
    int n = 0;
    if (x <= 0x00000000FFFFFFFFUL) { n += 32; x <<= 32; }
    if (x <= 0x0000FFFFFFFFFFFFUL) { n += 16; x <<= 16; }
    if (x <= 0x00FFFFFFFFFFFFFFUL) { n += 8;  x <<= 8;  }
    if (x <= 0x0FFFFFFFFFFFFFFFUL) { n += 4;  x <<= 4;  }
    if (x <= 0x3FFFFFFFFFFFFFFFUL) { n += 2;  x <<= 2;  }
    if (x <= 0x7FFFFFFFFFFFFFFFUL) { n += 1;  }
    return n;
}
static int clz32(u32 x) {
    int n = 0;
    if (x <= 0x0000FFFFU) { n += 16; x <<= 16; }
    if (x <= 0x00FFFFFFU) { n += 8;  x <<= 8;  }
    if (x <= 0x0FFFFFFFU) { n += 4;  x <<= 4;  }
    if (x <= 0x3FFFFFFFU) { n += 2;  x <<= 2;  }
    if (x <= 0x7FFFFFFFU) { n += 1;  }
    return n;
}

/* ================================================================
 * DOUBLE core: round & pack.
 *
 * Value = (-1)^sign * sig * 2^(exp - (52+SHIFT)) where sig is a 64-bit
 * significand whose leading (implicit) integer bit is intended to sit at
 * bit position (52+SHIFT). We keep SHIFT=... encoded implicitly by passing
 * `sig` already scaled so that the integer part occupies bit 52+guard bits.
 *
 * Concretely we adopt: incoming `sig` has the 53-bit significand in bits
 * [62:10] when normalized (leading 1 at bit 62), and bits [9:0] are the
 * round/sticky region. `exp` is the biased exponent that the leading-bit
 * position (bit 62) corresponds to. round_pack_d normalizes, rounds to 53
 * bits (round-nearest-even), handles overflow/underflow/subnormal.
 * ================================================================ */

/* Normalize+round a double. sig: leading significant bit should be near bit 62.
 * exp: biased exponent assuming leading 1 at bit 62. */
static u64 round_pack_d(int sign, int exp, u64 sig) {
    /* We want the significand as 53 bits (bit52..bit0) plus guard/round/sticky.
     * Bring leading bit to bit 62: value's units bit at bit 62 means the
     * fraction below occupies 62 bits; we keep 10 extra low bits for rounding.
     * Result mantissa = sig >> 10 (53 bits incl implicit), round bits = low 10.*/
    u64 sticky, round, roundbits;
    int shift;

    if (sig == 0) return ((u64)sign << 63); /* +/-0 */

    /* Normalize so leading 1 is at bit 62. */
    shift = clz64(sig) - 1;   /* bit63 is unused headroom; put msb at 62 */
    if (shift > 0) { sig <<= shift; exp -= shift; }
    else if (shift < 0) {
        /* msb above bit62: shift right, accumulate sticky */
        int s = -shift;
        sticky = (sig & ((1UL << s) - 1)) ? 1UL : 0UL;
        sig = (sig >> s) | sticky;
        exp += s;
    }

    /* Now leading 1 at bit62. 53-bit significand = bits [62:10]. */
    /* Handle subnormal/underflow: if exp <= 0 we must shift right (exp counts
     * biased position of bit62 == unbiased exponent + bias). For a normal the
     * smallest biased exp is 1. If exp < 1, denormalize. */
    if (exp <= 0) {
        int s = 1 - exp;          /* shift right this many to reach exp=1 range */
        if (s >= 64) {
            /* everything gone; sticky only */
            sig = (sig != 0) ? 1UL : 0UL;
        } else {
            sticky = (sig & ((1UL << s) - 1)) ? 1UL : 0UL;
            sig = (sig >> s) | sticky;
        }
        exp = 0;                  /* subnormal biased exponent field */
    }

    /* Round to nearest even on the low 10 bits. */
    roundbits = sig & 0x3ffUL;
    round = 0x200UL;              /* halfway = bit9 */
    {
        u64 mant = sig >> 10;     /* 53 bits (bit52 implicit for normal) */
        if (roundbits > round) mant += 1;
        else if (roundbits == round) { if (mant & 1UL) mant += 1; } /* tie->even */

        /* mant may have carried into bit53 (e.g. 0x1fff...->0x2000...) */
        if (mant & (1UL << 53)) { mant >>= 1; exp += 1; }
        /* Subnormal that rounded up to normal: bit52 set with exp==0 -> becomes
         * smallest normal automatically because we pack exp then add. Detect:
         * if exp==0 but mant has bit52 set, it's the smallest normal (exp field
         * should be 1). Packing exp=0 with mant bit52 set actually encodes
         * exactly that: field exp=0, but IEEE normal needs exp>=1. We must bump.*/
        if (exp == 0 && (mant & (1UL << 52))) {
            exp = 1;
            mant &= D_MANT_MASK;  /* drop implicit; field carries it via exp>=1 */
        }

        /* Overflow to infinity. */
        if (exp >= D_EXP_MAX) {
            return ((u64)sign << 63) | D_INF;
        }

        if (exp == 0) {
            /* subnormal: mant is the fraction (implicit bit already 0/absent) */
            return ((u64)sign << 63) | (mant & D_MANT_MASK);
        }
        return ((u64)sign << 63) | ((u64)exp << 52) | (mant & D_MANT_MASK);
    }
}

/* Unpack helpers */
struct dparts { int sign; int exp; u64 mant; }; /* mant WITHOUT implicit bit made explicit here */

/* classify */
static int d_is_nan(u64 x){ return ((x & D_INF)==D_INF) && (x & D_MANT_MASK); }
static int d_is_inf(u64 x){ return ((x & D_INF)==D_INF) && !(x & D_MANT_MASK); }

unsigned long __negdf2(unsigned long a){ return a ^ D_SIGN; }

/* ---------------- __adddf3 / __subdf3 ---------------- */
static u64 add_d(u64 a, u64 b) {
    int sa = (int)(a >> 63), sb = (int)(b >> 63);
    int ea = (int)((a >> 52) & D_EXP_MAX), eb = (int)((b >> 52) & D_EXP_MAX);
    u64 ma = a & D_MANT_MASK, mb = b & D_MANT_MASK;
    u64 siga, sigb;
    int exp, sign;

    /* NaN */
    if (d_is_nan(a) || d_is_nan(b)) return D_QNAN | ((a|b) & D_SIGN & 0); /* canonical */
    /* Inf */
    if (ea == D_EXP_MAX || eb == D_EXP_MAX) {
        if (ea == D_EXP_MAX && eb == D_EXP_MAX) {
            if (sa != sb) return D_QNAN;      /* inf - inf */
            return a;                          /* same-sign inf */
        }
        return (ea == D_EXP_MAX) ? a : b;
    }

    /* build significands with implicit bit; place leading bit at bit 62.
     * A normal's significand (implicit|frac) is 53 bits (bit52..0). Shift left
     * by 10 so leading 1 lands at bit 62 and we retain 10 rounding bits. */
    if (ea == 0) { siga = ma << 10; ea = 1; } else { siga = (ma | (1UL<<52)) << 10; }
    if (eb == 0) { sigb = mb << 10; eb = 1; } else { sigb = (mb | (1UL<<52)) << 10; }
    /* Now value = sig * 2^(e - 1023 - 62). Biased exponent of bit62 is e. */

    /* Align to larger exponent. */
    if (ea >= eb) {
        int d = ea - eb; exp = ea;
        if (d) {
            u64 sticky = (d < 64) ? ((sigb & ((1UL<<d)-1)) ? 1UL:0UL)
                                  : (sigb ? 1UL:0UL);
            sigb = (d < 64) ? (sigb >> d) : 0UL;
            sigb |= sticky;
        }
    } else {
        int d = eb - ea; exp = eb;
        u64 sticky = (d < 64) ? ((siga & ((1UL<<d)-1)) ? 1UL:0UL)
                              : (siga ? 1UL:0UL);
        siga = (d < 64) ? (siga >> d) : 0UL;
        siga |= sticky;
    }

    if (sa == sb) {
        sign = sa;
        u64 sig = siga + sigb;    /* may set bit63 (carry) */
        return round_pack_d(sign, exp, sig);
    } else {
        /* subtraction */
        u64 sig;
        if (siga >= sigb) { sig = siga - sigb; sign = sa; }
        else              { sig = sigb - siga; sign = sb; }
        if (sig == 0) return 0UL; /* exact cancellation -> +0 (round nearest) */
        return round_pack_d(sign, exp, sig);
    }
}

unsigned long __adddf3(unsigned long a, unsigned long b){ return add_d(a,b); }
unsigned long __subdf3(unsigned long a, unsigned long b){ return add_d(a, b ^ D_SIGN); }

/* ---------------- __muldf3 ---------------- */
/* 64x64 -> 128 via 32-bit limbs. Returns hi in *hi, lo returned. */
static u64 mul64(u64 x, u64 y, u64 *hi) {
    u64 xl = x & 0xffffffffUL, xh = x >> 32;
    u64 yl = y & 0xffffffffUL, yh = y >> 32;
    u64 ll = xl*yl, lh = xl*yh, hl = xh*yl, hh = xh*yh;
    u64 mid = (ll >> 32) + (lh & 0xffffffffUL) + (hl & 0xffffffffUL);
    u64 lo  = (ll & 0xffffffffUL) | (mid << 32);
    *hi = hh + (lh >> 32) + (hl >> 32) + (mid >> 32);
    return lo;
}

unsigned long __muldf3(unsigned long a, unsigned long b) {
    int sa=(int)(a>>63), sb=(int)(b>>63);
    int ea=(int)((a>>52)&D_EXP_MAX), eb=(int)((b>>52)&D_EXP_MAX);
    u64 ma=a&D_MANT_MASK, mb=b&D_MANT_MASK;
    int sign = sa ^ sb;

    if (d_is_nan(a)||d_is_nan(b)) return D_QNAN;
    if (ea==D_EXP_MAX || eb==D_EXP_MAX) {
        /* inf */
        if (d_is_inf(a)) { if ((eb==0)&&mb==0) return D_QNAN; return ((u64)sign<<63)|D_INF; }
        if (d_is_inf(b)) { if ((ea==0)&&ma==0) return D_QNAN; return ((u64)sign<<63)|D_INF; }
    }
    /* zero */
    if ((ea==0&&ma==0)||(eb==0&&mb==0)) return (u64)sign<<63;

    /* significands (53-bit) and exponents */
    u64 siga, sigb; int expa, expb;
    if (ea==0){ int s=clz64(ma)-11; siga=ma<<s; expa=1-s; } /* normalize subnormal: leading to bit52 */
    else      { siga=ma|(1UL<<52); expa=ea; }
    if (eb==0){ int s=clz64(mb)-11; sigb=mb<<s; expb=1-s; }
    else      { sigb=mb|(1UL<<52); expb=eb; }

    /* product of two 53-bit numbers: up to 106 bits, leading at bit104 or 105 */
    u64 hi, lo; lo = mul64(siga, sigb, &hi);
    /* full 128-bit value = hi:lo, but only low 106 bits used. Collapse to 64
     * with sticky. We want to feed round_pack_d with leading bit at bit62.
     * The product leading bit is at bit 104 or 105 (since each sig has bit52).
     * Exponent of the product's bit (unit at bit104..) : value = product *
     * 2^(expa-1075 + expb-1075) ... let's track biased exponent of bit62. */
    /* Combine hi:lo into a 64-bit sig with sticky in bit0, leading bit at 62. */
    /* Determine position of msb of 128-bit number. */
    int msb; u64 sig;
    if (hi) { msb = 127 - clz64(hi); }
    else    { msb = 63 - clz64(lo); }
    /* We want msb moved to bit 62. shift right by (msb-62). */
    {
        int sh = msb - 62;
        u64 s_sticky;
        /* gather sticky = any bit below (sh) position of the 128-bit value */
        if (sh <= 0) {
            /* shift left (product small only if subnormals) */
            int l = -sh;
            sig = (l>=64)?0:(lo << l);      /* hi ignored since msb<63 here means hi==0 */
            s_sticky = 0;
        } else if (sh < 64) {
            s_sticky = (lo & ((1UL<<sh)-1)) ? 1UL:0UL;
            sig = (lo >> sh) | (hi << (64 - sh));
            sig |= s_sticky;
        } else {
            int sh2 = sh - 64;
            s_sticky = (lo != 0) ? 1UL:0UL;
            if (sh2) s_sticky |= (hi & ((1UL<<sh2)-1)) ? 1UL:0UL;
            sig = (sh2>=64)?0:(hi >> sh2);
            sig |= s_sticky;
        }
    }
    /* biased exponent of bit62: product value = sig62 * 2^(exp-1023-62).
     * The product P (integer) = product; P = sig62 * 2^(msb-62)*... Let's
     * compute exp such that leading bit (bit62 now) has biased exp.
     * Unbiased exponent of the product's MSB: (expa-1023 + expb-1023) counts
     * with each sig having value ~2^52..2^53. siga ~ 2^52 * m, product ~2^104.
     * value = product * 2^(expa-1023-52 + expb-1023-52).
     * After moving msb(bit `msb`) to bit62, sig62 = product>>(msb-62) approx,
     * value = sig62 * 2^( (msb-62) + expa+expb-1023-1023-104 ).
     * biased exp of bit62 = exponent + 1023 where value=sig62*2^(exp-... ) ...
     * Simpler: define E = biased exp of bit62. value = sig62 * 2^(E-1023-62).
     * => E-1023-62 = (msb-62) + (expa+expb -2*1023 -104)
     * => E = expa+expb -1023 -104 +62 +1023  ... let me just compute:
     */
    {
        int E = expa + expb - D_EXP_BIAS + (msb - 104);
        /* derivation: with both normalized to bit52 (value=sig*2^(e-1023-52)),
         * product integer has value siga*sigb, whose true value multiplier is
         * 2^((expa-1023-52)+(expb-1023-52)) = 2^(expa+expb-2*1023-104).
         * We hold sig62 (msb at bit62) = product>>(msb-62). So
         * value = sig62 * 2^(msb-62) * 2^(expa+expb-2046-104)
         *       = sig62 * 2^(expa+expb-2046-104+msb-62).
         * round_pack_d expects value = sig * 2^(exp-1023-62), i.e.
         * exp-1023-62 = expa+expb-2046-104+msb-62
         * exp = expa+expb-2046-104+msb-62 +1023+62 = expa+expb+msb-104-1023. */
        E = expa + expb + msb - 104 - D_EXP_BIAS;
        return round_pack_d(sign, E, sig);
    }
}

/* ---------------- __divdf3 ---------------- */
unsigned long __divdf3(unsigned long a, unsigned long b) {
    int sa=(int)(a>>63), sb=(int)(b>>63);
    int ea=(int)((a>>52)&D_EXP_MAX), eb=(int)((b>>52)&D_EXP_MAX);
    u64 ma=a&D_MANT_MASK, mb=b&D_MANT_MASK;
    int sign=sa^sb;

    if (d_is_nan(a)||d_is_nan(b)) return D_QNAN;
    if (d_is_inf(a)) { if (d_is_inf(b)) return D_QNAN; return ((u64)sign<<63)|D_INF; }
    if (d_is_inf(b)) { return (u64)sign<<63; } /* finite/inf = 0 */
    /* a is inf handled; now b zero? */
    if (eb==0 && mb==0) {
        if (ea==0 && ma==0) return D_QNAN; /* 0/0 */
        return ((u64)sign<<63)|D_INF;       /* x/0 -> inf */
    }
    if (ea==0 && ma==0) return (u64)sign<<63; /* 0/x -> 0 */

    u64 siga, sigb; int expa, expb;
    if (ea==0){ int s=clz64(ma)-11; siga=ma<<s; expa=1-s; }
    else      { siga=ma|(1UL<<52); expa=ea; }
    if (eb==0){ int s=clz64(mb)-11; sigb=mb<<s; expb=1-s; }
    else      { sigb=mb|(1UL<<52); expb=eb; }

    /* Both siga,sigb are 53-bit (leading bit52). Compute quotient with enough
     * fractional bits. We want result leading bit at bit62 for round_pack_d.
     * Do restoring long division: numerator scaled so quotient has ~62-bit
     * fraction. siga/sigb is in [0.5,2). Shift numerator left so quotient msb
     * lands near bit62. */
    /* Q = (siga << K) / sigb, with remainder sticky. Choose K so quotient ~2^62.
     * siga,sigb ~2^52, ratio ~1, so shift by ~62 gives quotient ~2^62. */
    u64 rem = siga, q = 0;
    int i;
    int K = 62;
    /* long division producing K+1 quotient bits */
    for (i = 0; i <= K; i++) {
        q <<= 1;
        if (rem >= sigb) { rem -= sigb; q |= 1; }
        /* shift remainder left, but cap: rem<sigb<2^53, rem<<1<2^54 safe */
        rem <<= 1;
    }
    /* now q has K+1 bits, value approx (siga/sigb)*2^K. sticky if rem!=0 */
    u64 sig = q | (rem ? 1UL : 0UL);
    /* value = (siga/sigb) with siga=... value_a = siga*2^(expa-1023-52),
     * value_b likewise. ratio = (siga/sigb)*2^(expa-expb).
     * q ~ (siga/sigb)*2^K. so ratio = q*2^(expa-expb-K).
     * round_pack_d wants sig*2^(exp-1023-62). q's msb ~ bit62 (since ratio~1
     * and K=62 -> q~2^62). Set exp so exp-1023-62 = expa-expb-K.
     * exp = expa-expb-K +1023+62 = expa-expb-62+1023+62 = expa-expb+1023. */
    int E = expa - expb + D_EXP_BIAS;
    return round_pack_d(sign, E, sig);
}

/* ---------------- double compares ---------------- */
/* returns: -1,0,1 for a<b,a==b,a>b ; 2 if unordered */
static int cmp_d(u64 a, u64 b) {
    if (d_is_nan(a) || d_is_nan(b)) return 2;
    /* +0 == -0 */
    if (((a | b) << 1) == 0) return 0; /* both zero (any sign) */
    int sa=(int)(a>>63), sb=(int)(b>>63);
    if (sa != sb) return sa ? -1 : 1;  /* negative < positive */
    /* same sign: compare magnitude via the ordered integer trick */
    if (a == b) return 0;
    if (!sa) return (a < b) ? -1 : 1;      /* positive: larger bits = larger */
    else     return (a < b) ? 1 : -1;      /* negative: larger bits = smaller */
}
int __unorddf2(unsigned long a, unsigned long b){ return (d_is_nan(a)||d_is_nan(b))?1:0; }
int __eqdf2(unsigned long a, unsigned long b){ int c=cmp_d(a,b); return (c==0)?0:1; }
int __nedf2(unsigned long a, unsigned long b){ int c=cmp_d(a,b); return (c==0)?0:1; }
int __ltdf2(unsigned long a, unsigned long b){ int c=cmp_d(a,b); return (c==2)?1:c; }
int __ledf2(unsigned long a, unsigned long b){ int c=cmp_d(a,b); return (c==2)?1:c; }
int __gtdf2(unsigned long a, unsigned long b){ int c=cmp_d(a,b); return (c==2)?-1:c; }
int __gedf2(unsigned long a, unsigned long b){ int c=cmp_d(a,b); return (c==2)?-1:c; }

/* ---------------- double <-> int ---------------- */
unsigned long __floatsidf(int a) {
    if (a == 0) return 0UL;
    int sign = 0; u64 mag;
    if (a < 0) { sign = 1; mag = (u64)(-(i64)a); } else mag = (u64)a;
    /* value = mag; feed round_pack_d with leading bit at bit62.
     * exp for integer: value = sig*2^(exp-1023-62) with sig=mag<<sh so msb at62*/
    int msb = 63 - clz64(mag);
    int sh = 62 - msb;
    u64 sig = (sh>=0)? (mag << sh) : (mag >> (-sh));
    int E = msb + D_EXP_BIAS;  /* value = mag*2^0 = sig*2^(-sh); exp-1023-62=-sh => exp=1023+62-sh=1023+62-(62-msb)=1023+msb */
    return round_pack_d(sign, E, sig);
}
unsigned long __floatunsidf(unsigned a) {
    if (a == 0) return 0UL;
    u64 mag = (u64)a;
    int msb = 63 - clz64(mag);
    int sh = 62 - msb;
    u64 sig = (sh>=0)?(mag<<sh):(mag>>(-sh));
    int E = msb + D_EXP_BIAS;
    return round_pack_d(0, E, sig);
}
/* double -> signed int, truncate toward zero. Out of range -> matches x86
 * cvttsd2si style: gcc's __fixdfsi saturates; hardware (int)d is INT_MIN for
 * overflow/NaN. We follow libgcc: clamp to INT_MIN/INT_MAX. */
int __fixdfsi(unsigned long a) {
    int sign=(int)(a>>63);
    int e=(int)((a>>52)&D_EXP_MAX);
    u64 m=a&D_MANT_MASK;
    if (e==D_EXP_MAX) { /* inf/nan */
        if (m) return 0;                 /* NaN -> 0 (libgcc) */
        return sign ? (-2147483647-1) : 2147483647;
    }
    int ue = e - D_EXP_BIAS;             /* unbiased */
    if (ue < 0) return 0;                /* |x|<1 truncates to 0 */
    if (ue > 30) {                       /* magnitude >= 2^31 -> overflow */
        return sign ? (-2147483647-1) : 2147483647;
    }
    u64 sig = m | (1UL<<52);             /* 1.m in 53-bit */
    /* integer part = sig >> (52 - ue) */
    u64 ip = sig >> (52 - ue);
    if (sign) return (int)(-(i64)ip); else return (int)ip;
}
unsigned __fixunsdfsi(unsigned long a) {
    int sign=(int)(a>>63);
    int e=(int)((a>>52)&D_EXP_MAX);
    u64 m=a&D_MANT_MASK;
    if (e==D_EXP_MAX) { if (m) return 0; return sign?0u:0xffffffffu; }
    if (sign) return 0;                  /* negatives -> 0 (libgcc) */
    int ue = e - D_EXP_BIAS;
    if (ue < 0) return 0;
    if (ue > 31) return 0xffffffffu;
    u64 sig = m | (1UL<<52);
    u64 ip = sig >> (52 - ue);
    return (unsigned)ip;
}

/* ================================================================
 * FLOAT (binary32) -- same structure, 32-bit fields, leading bit at bit30
 * of a 32-bit... but we reuse 64-bit math for simplicity & then round to 24
 * bits. We build float significand in a u64 with leading bit at bit62 and a
 * dedicated float round_pack.
 * ================================================================ */
static u32 round_pack_f(int sign, int exp, u64 sig) {
    /* value = sig * 2^(exp-127-62), leading bit target bit62. Round to 24-bit
     * significand: mantissa = sig >> (62-23) = sig>>39, round bits low 39. */
    u64 sticky; int shift;
    if (sig == 0) return ((u32)sign<<31);
    shift = clz64(sig) - 1;
    if (shift>0){ sig<<=shift; exp-=shift; }
    else if (shift<0){ int s=-shift; sticky=(sig&((1UL<<s)-1))?1:0; sig=(sig>>s)|sticky; exp+=s; }

    if (exp <= 0) {
        int s = 1 - exp;
        if (s >= 64) sig = (sig!=0)?1:0;
        else { sticky=(sig&((1UL<<s)-1))?1:0; sig=(sig>>s)|sticky; }
        exp = 0;
    }
    {
        u64 half = (1UL<<38);
        u64 roundbits = sig & ((1UL<<39)-1);
        u64 mant = sig >> 39;      /* 24 bits */
        if (roundbits > half) mant+=1;
        else if (roundbits==half){ if (mant&1) mant+=1; }
        if (mant & (1UL<<24)) { mant>>=1; exp+=1; }
        if (exp==0 && (mant&(1UL<<23))) { exp=1; mant&=F_MANT_MASK; }
        if (exp >= F_EXP_MAX) return ((u32)sign<<31)|F_INF;
        if (exp==0) return ((u32)sign<<31)|((u32)(mant & F_MANT_MASK));
        return ((u32)sign<<31)|((u32)exp<<23)|((u32)(mant & F_MANT_MASK));
    }
}

static int f_is_nan(u32 x){ return ((x & F_INF)==F_INF) && (x & F_MANT_MASK); }
static int f_is_inf(u32 x){ return ((x & F_INF)==F_INF) && !(x & F_MANT_MASK); }

unsigned __negsf2(unsigned a){ return a ^ F_SIGN; }

static u32 add_f(u32 a, u32 b) {
    int sa=(int)(a>>31), sb=(int)(b>>31);
    int ea=(int)((a>>23)&F_EXP_MAX), eb=(int)((b>>23)&F_EXP_MAX);
    u32 ma=a&F_MANT_MASK, mb=b&F_MANT_MASK;
    u64 siga, sigb; int exp, sign;

    if (f_is_nan(a)||f_is_nan(b)) return F_QNAN;
    if (ea==F_EXP_MAX || eb==F_EXP_MAX) {
        if (ea==F_EXP_MAX && eb==F_EXP_MAX){ if(sa!=sb) return F_QNAN; return a; }
        return (ea==F_EXP_MAX)?a:b;
    }
    /* significand 24-bit (bit23..0). Shift into u64 leading bit at bit62:
     * put bit23 at bit62 -> shift left by 39. */
    if (ea==0){ siga=((u64)ma)<<39; ea=1; } else { siga=((u64)(ma|(1U<<23)))<<39; }
    if (eb==0){ sigb=((u64)mb)<<39; eb=1; } else { sigb=((u64)(mb|(1U<<23)))<<39; }

    if (ea>=eb){ int d=ea-eb; exp=ea; if(d){ u64 st=(d<64)?((sigb&((1UL<<d)-1))?1:0):(sigb?1:0); sigb=(d<64)?(sigb>>d):0; sigb|=st; } }
    else       { int d=eb-ea; exp=eb; u64 st=(d<64)?((siga&((1UL<<d)-1))?1:0):(siga?1:0); siga=(d<64)?(siga>>d):0; siga|=st; }

    if (sa==sb){ sign=sa; return round_pack_f(sign, exp, siga+sigb); }
    else {
        u64 sig; if (siga>=sigb){ sig=siga-sigb; sign=sa; } else { sig=sigb-siga; sign=sb; }
        if (sig==0) return 0u;
        return round_pack_f(sign, exp, sig);
    }
}
unsigned __addsf3(unsigned a, unsigned b){ return add_f(a,b); }
unsigned __subsf3(unsigned a, unsigned b){ return add_f(a, b ^ F_SIGN); }

unsigned __mulsf3(unsigned a, unsigned b) {
    int sa=(int)(a>>31), sb=(int)(b>>31);
    int ea=(int)((a>>23)&F_EXP_MAX), eb=(int)((b>>23)&F_EXP_MAX);
    u32 ma=a&F_MANT_MASK, mb=b&F_MANT_MASK;
    int sign=sa^sb;
    if (f_is_nan(a)||f_is_nan(b)) return F_QNAN;
    if (ea==F_EXP_MAX||eb==F_EXP_MAX){
        if (f_is_inf(a)){ if(eb==0&&mb==0) return F_QNAN; return ((u32)sign<<31)|F_INF; }
        if (f_is_inf(b)){ if(ea==0&&ma==0) return F_QNAN; return ((u32)sign<<31)|F_INF; }
    }
    if ((ea==0&&ma==0)||(eb==0&&mb==0)) return (u32)sign<<31;
    u64 siga,sigb; int expa,expb;
    if (ea==0){ int s=clz32(ma)-8; siga=((u64)ma)<<s; expa=1-s; } else { siga=ma|(1U<<23); expa=ea; }
    if (eb==0){ int s=clz32(mb)-8; sigb=((u64)mb)<<s; expb=1-s; } else { sigb=mb|(1U<<23); expb=eb; }
    /* 24x24 = up to 48-bit product, fits in u64. leading bit at 46 or 47. */
    u64 prod = siga * sigb;
    int msb = 63 - clz64(prod);
    /* move msb to bit62 */
    int sh = 62 - msb;
    u64 sig = (sh>=0)?(prod<<sh):((prod>>(-sh)) | ((prod&((1UL<<(-sh))-1))?1:0));
    /* exp: analogous to double. value=prod*2^((expa-127-23)+(expb-127-23)).
     * sig=prod<<sh(=62-msb). value=sig*2^(-sh)*2^(expa+expb-254-46).
     * want sig*2^(exp-127-62): exp-127-62 = -sh+expa+expb-254-46
     * exp = expa+expb-254-46 -(62-msb)+127+62 = expa+expb+msb-46-127... check:
     * = expa+expb-254-46-62+msb+189 = expa+expb+msb-173. And 173=46+127. */
    int E = expa+expb+msb-46-F_EXP_BIAS;
    return round_pack_f(sign, E, sig);
}

unsigned __divsf3(unsigned a, unsigned b) {
    int sa=(int)(a>>31), sb=(int)(b>>31);
    int ea=(int)((a>>23)&F_EXP_MAX), eb=(int)((b>>23)&F_EXP_MAX);
    u32 ma=a&F_MANT_MASK, mb=b&F_MANT_MASK;
    int sign=sa^sb;
    if (f_is_nan(a)||f_is_nan(b)) return F_QNAN;
    if (f_is_inf(a)){ if(f_is_inf(b)) return F_QNAN; return ((u32)sign<<31)|F_INF; }
    if (f_is_inf(b)) return (u32)sign<<31;
    if (eb==0&&mb==0){ if(ea==0&&ma==0) return F_QNAN; return ((u32)sign<<31)|F_INF; }
    if (ea==0&&ma==0) return (u32)sign<<31;
    u64 siga,sigb; int expa,expb;
    if (ea==0){ int s=clz32(ma)-8; siga=((u64)ma)<<s; expa=1-s; } else { siga=ma|(1U<<23); expa=ea; }
    if (eb==0){ int s=clz32(mb)-8; sigb=((u64)mb)<<s; expb=1-s; } else { sigb=mb|(1U<<23); expb=eb; }
    /* quotient with ~62-bit fraction. siga,sigb ~2^23, ratio~1. Shift num so q~2^62. */
    u64 rem=siga, q=0; int i; int K=62;
    /* need rem<<1 not to overflow: siga<2^24, but we shift each step; rem<sigb<2^24 so rem<<1<2^25 safe */
    for (i=0;i<=K;i++){ q<<=1; if(rem>=sigb){ rem-=sigb; q|=1; } rem<<=1; }
    u64 sig = q | (rem?1UL:0UL);
    int E = expa - expb + F_EXP_BIAS;
    return round_pack_f(sign, E, sig);
}

static int cmp_f(u32 a, u32 b) {
    if (f_is_nan(a)||f_is_nan(b)) return 2;
    if (((a|b)<<1)==0) return 0;
    int sa=(int)(a>>31), sb=(int)(b>>31);
    if (sa!=sb) return sa?-1:1;
    if (a==b) return 0;
    if (!sa) return (a<b)?-1:1; else return (a<b)?1:-1;
}
int __unordsf2(unsigned a, unsigned b){ return (f_is_nan(a)||f_is_nan(b))?1:0; }
int __eqsf2(unsigned a, unsigned b){ int c=cmp_f(a,b); return (c==0)?0:1; }
int __nesf2(unsigned a, unsigned b){ int c=cmp_f(a,b); return (c==0)?0:1; }
int __ltsf2(unsigned a, unsigned b){ int c=cmp_f(a,b); return (c==2)?1:c; }
int __lesf2(unsigned a, unsigned b){ int c=cmp_f(a,b); return (c==2)?1:c; }
int __gtsf2(unsigned a, unsigned b){ int c=cmp_f(a,b); return (c==2)?-1:c; }
int __gesf2(unsigned a, unsigned b){ int c=cmp_f(a,b); return (c==2)?-1:c; }

unsigned __floatsisf(int a) {
    if (a==0) return 0u;
    int sign=0; u64 mag;
    if (a<0){ sign=1; mag=(u64)(-(i64)a); } else mag=(u64)a;
    int msb=63-clz64(mag); int sh=62-msb;
    u64 sig=(sh>=0)?(mag<<sh):(mag>>(-sh));
    int E=msb+F_EXP_BIAS;
    return round_pack_f(sign,E,sig);
}
unsigned __floatunsisf(unsigned a) {
    if (a==0) return 0u;
    u64 mag=(u64)a; int msb=63-clz64(mag); int sh=62-msb;
    u64 sig=(sh>=0)?(mag<<sh):(mag>>(-sh));
    int E=msb+F_EXP_BIAS;
    return round_pack_f(0,E,sig);
}
int __fixsfsi(unsigned a) {
    int sign=(int)(a>>31); int e=(int)((a>>23)&F_EXP_MAX); u32 m=a&F_MANT_MASK;
    if (e==F_EXP_MAX){ if(m) return 0; return sign?(-2147483647-1):2147483647; }
    int ue=e-F_EXP_BIAS;
    if (ue<0) return 0;
    if (ue>30) return sign?(-2147483647-1):2147483647;
    u64 sig=(u64)(m|(1U<<23));
    u64 ip=(ue>=23)?(sig<<(ue-23)):(sig>>(23-ue));
    return sign?(int)(-(i64)ip):(int)ip;
}
unsigned __fixunssfsi(unsigned a) {
    int sign=(int)(a>>31); int e=(int)((a>>23)&F_EXP_MAX); u32 m=a&F_MANT_MASK;
    if (e==F_EXP_MAX){ if(m) return 0; return sign?0u:0xffffffffu; }
    if (sign) return 0;
    int ue=e-F_EXP_BIAS;
    if (ue<0) return 0;
    if (ue>31) return 0xffffffffu;
    u64 sig=(u64)(m|(1U<<23));
    u64 ip=(ue>=23)?(sig<<(ue-23)):(sig>>(23-ue));
    return (unsigned)ip;
}

/* ---------------- float <-> double ---------------- */
unsigned long __extendsfdf2(unsigned a) {
    int sign=(int)(a>>31); int e=(int)((a>>23)&F_EXP_MAX); u32 m=a&F_MANT_MASK;
    if (e==F_EXP_MAX){ if(m) return D_QNAN; return ((u64)sign<<63)|D_INF; }
    if (e==0){
        if (m==0) return (u64)sign<<63;   /* +/-0 */
        /* subnormal float -> normalized double (always representable) */
        int s=clz32(m)-8;                 /* bring bit23 to leading */
        u32 mm = m<<s;                    /* now bit23 set */
        int exp = 1 - s;                  /* unbiased+bias for float: e_field=1-s conceptually */
        /* float value = m*2^(1-127-23). Normalized: sig=mm(bit23) value=mm*2^(exp-127-23). */
        u64 mant = ((u64)(mm & F_MANT_MASK)) << (D_MANT_BITS - F_MANT_BITS);
        int de = (exp - F_EXP_BIAS) + D_EXP_BIAS;
        return ((u64)sign<<63)|((u64)de<<52)|mant;
    }
    /* normal: exact widening */
    u64 mant = ((u64)m) << (D_MANT_BITS - F_MANT_BITS);
    int de = (e - F_EXP_BIAS) + D_EXP_BIAS;
    return ((u64)sign<<63)|((u64)de<<52)|mant;
}
unsigned __truncdfsf2(unsigned long a) {
    int sign=(int)(a>>63); int e=(int)((a>>52)&D_EXP_MAX); u64 m=a&D_MANT_MASK;
    if (e==D_EXP_MAX){ if(m) return F_QNAN; return ((u32)sign<<31)|F_INF; }
    if (e==0 && m==0) return (u32)sign<<31;
    /* Build significand leading bit at bit62 and round to float. */
    u64 sig; int expd;
    if (e==0){ int s=clz64(m)-11; sig=(m<<s)<<10; expd=1-s; } /* subnormal double */
    else { sig=(m|(1UL<<52))<<10; expd=e; }
    /* value = sig*2^(expd-1023-62). round_pack_f wants sig*2^(exp-127-62).
     * so exp-127 = expd-1023 => exp = expd-1023+127. */
    int E = expd - D_EXP_BIAS + F_EXP_BIAS;
    return round_pack_f(sign, E, sig);
}
