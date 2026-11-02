struct P { globs: Vec<String>, mpfx: String }
impl P {
    fn gres_ty(&self, n:&str)->String{
        let mut i=0;
        while i<self.globs.len(){
            let b=self.globs[i].as_bytes(); let mut cut=0; let mut k=0;
            while k<b.len(){ if b[k]==124 { cut=k; k=b.len(); } else { k=k+1; } }
            if String::from(&self.globs[i][..cut]).as_str()==self.mpfx.as_str() {
                let mut q=String::from(&self.globs[i][(cut+1)..]); q.push('$'); q.push_str(n);
                return q; }
            i=i+1; }
        String::from("") }
}
fn main(){
    let mut g:Vec<String>=Vec::new();
    g.push(String::from("zz|first")); g.push(String::from("mm|second")); g.push(String::from("qq|third"));
    let p=P{ globs:g, mpfx:String::from("qq") };
    println!("[{}]", p.gres_ty("name"));
}
