#!/usr/bin/env node
const childProcess = require("child_process");
const path = require("path");

const bodyPath = process.argv[2];
const identifier = process.argv[3];
if (!bodyPath || !identifier) {
  process.stderr.write("usage: write-derived-wrapper-prefix-m0.js <body> <identifier>\n");
  process.exit(1);
}

const prefix = childProcess.execFileSync(
  process.execPath,
  [path.join(__dirname, "emit-derived-wrapper-prefix.js"), bodyPath, identifier]
);

const headerFields = [
  ["mh_magic", 0, 4],
  ["mh_cputype", 4, 8],
  ["mh_cpusubtype", 8, 12],
  ["mh_filetype", 12, 16],
  ["mh_ncmds", 16, 20],
  ["mh_sizeofcmds", 20, 24],
  ["mh_flags", 24, 28],
  ["mh_reserved", 28, 32],
];

const segmentFields = [
  ["cmd", 0, 4],
  ["cmdsize", 4, 8],
  ["segname", 8, 24],
  ["vmaddr", 24, 32],
  ["vmsize", 32, 40],
  ["fileoff", 40, 48],
  ["filesize", 48, 56],
  ["maxprot", 56, 60],
  ["initprot", 60, 64],
  ["nsects", 64, 68],
  ["flags", 68, 72],
];

const segmentCommands = [
  ["lc_segment_pagezero", 32],
  ["lc_segment_text", 104],
];

const linkeditSegmentCommand = ["lc_segment_linkedit", 256];

const sectionFields = [
  ["sectname", 0, 16],
  ["segname", 16, 32],
  ["addr", 32, 40],
  ["size", 40, 48],
  ["offset", 48, 52],
  ["align", 52, 56],
  ["reloff", 56, 60],
  ["nreloc", 60, 64],
  ["flags", 64, 68],
  ["reserved1", 68, 72],
  ["reserved2", 72, 76],
  ["reserved3", 76, 80],
];

const linkeditDataFields = [
  ["cmd", 0, 4],
  ["cmdsize", 4, 8],
  ["dataoff", 8, 12],
  ["datasize", 12, 16],
];

const linkeditDataCommands = [
  ["lc_dyld_chained_fixups", 328],
  ["lc_dyld_exports_trie", 344],
];

const trailingLinkeditDataCommands = [
  ["lc_function_starts", 632],
  ["lc_data_in_code", 648],
  ["lc_code_signature", 664],
];

const symtabFields = [
  ["cmd", 0, 4],
  ["cmdsize", 4, 8],
  ["symoff", 8, 12],
  ["nsyms", 12, 16],
  ["stroff", 16, 20],
  ["strsize", 20, 24],
];

const dysymtabFields = [
  ["cmd", 0, 4],
  ["cmdsize", 4, 8],
  ["ilocalsym", 8, 12],
  ["nlocalsym", 12, 16],
  ["iextdefsym", 16, 20],
  ["nextdefsym", 20, 24],
  ["iundefsym", 24, 28],
  ["nundefsym", 28, 32],
  ["tocoff", 32, 36],
  ["ntoc", 36, 40],
  ["modtaboff", 40, 44],
  ["nmodtab", 44, 48],
  ["extrefsymoff", 48, 52],
  ["nextrefsyms", 52, 56],
  ["indirectsymoff", 56, 60],
  ["nindirectsyms", 60, 64],
  ["extreloff", 64, 68],
  ["nextrel", 68, 72],
  ["locreloff", 72, 76],
  ["nlocrel", 76, 80],
];

const stringCommandFields = [
  ["cmd", 0, 4],
  ["cmdsize", 4, 8],
  ["nameoff", 8, 12],
];

const uuidFields = [
  ["cmd", 0, 4],
  ["cmdsize", 4, 8],
  ["uuid", 8, 24],
];

const buildVersionFields = [
  ["cmd", 0, 4],
  ["cmdsize", 4, 8],
  ["platform", 8, 12],
  ["minos", 12, 16],
  ["sdk", 16, 20],
  ["ntools", 20, 24],
];

const sourceVersionFields = [
  ["cmd", 0, 4],
  ["cmdsize", 4, 8],
  ["version", 8, 16],
];

const mainFields = [
  ["cmd", 0, 4],
  ["cmdsize", 4, 8],
  ["entryoff", 8, 16],
  ["stacksize", 16, 24],
];

const dylibFields = [
  ["cmd", 0, 4],
  ["cmdsize", 4, 8],
  ["nameoff", 8, 12],
  ["timestamp", 12, 16],
  ["current_version", 16, 20],
  ["compatibility_version", 20, 24],
  ["name", 24, 56],
];

const regions = [
  ["load_command_padding", 680, 688],
];

function emitRegion(name, bytes) {
  process.stdout.write(`\n# prefix region: ${name}\n`);
  for (let i = 0; i < bytes.length; i += 16) {
    const chunk = bytes.subarray(i, i + 16).toString("hex");
    const macro = `${name}_${String(i / 16).padStart(2, "0")}`;
    process.stdout.write(`%${macro} ${chunk}\n`);
    process.stdout.write(`${macro}\n`);
  }
}

function emitHeaderField(name, bytes) {
  process.stdout.write(`%${name} ${bytes.toString("hex")}\n`);
  process.stdout.write(`${name}\n`);
}

function emitField(name, bytes) {
  process.stdout.write(`%${name} ${bytes.toString("hex")}\n`);
  process.stdout.write(`${name}\n`);
}

function emitSectionFields(sectionName, base) {
  process.stdout.write(`\n# prefix region: ${sectionName}\n`);
  for (const [fieldName, relStart, relEnd] of sectionFields) {
    emitField(`${sectionName}_${fieldName}`, prefix.subarray(base + relStart, base + relEnd));
  }
}

function emitLinkeditDataCommand(commandName, base) {
  process.stdout.write(`\n# prefix region: ${commandName}\n`);
  for (const [fieldName, relStart, relEnd] of linkeditDataFields) {
    emitField(`${commandName}_${fieldName}`, prefix.subarray(base + relStart, base + relEnd));
  }
}

function emitSymtabCommand(commandName, base) {
  process.stdout.write(`\n# prefix region: ${commandName}\n`);
  for (const [fieldName, relStart, relEnd] of symtabFields) {
    emitField(`${commandName}_${fieldName}`, prefix.subarray(base + relStart, base + relEnd));
  }
}

function emitFields(commandName, base, fields) {
  process.stdout.write(`\n# prefix region: ${commandName}\n`);
  for (const [fieldName, relStart, relEnd] of fields) {
    emitField(`${commandName}_${fieldName}`, prefix.subarray(base + relStart, base + relEnd));
  }
}

function emitPadding(name, start, end) {
  process.stdout.write(`\n# prefix region: ${name}\n`);
  emitField(name, prefix.subarray(start, end));
}

process.stdout.write("# M0 constructor source for derived Mach-O prefix\n");
process.stdout.write(`# identifier: ${identifier}\n`);

process.stdout.write("\n# prefix region: mach_header_64\n");
for (const [name, start, end] of headerFields) {
  emitHeaderField(name, prefix.subarray(start, end));
}

for (const [segmentName, base] of segmentCommands) {
  process.stdout.write(`\n# prefix region: ${segmentName}\n`);
  for (const [fieldName, relStart, relEnd] of segmentFields) {
    emitField(`${segmentName}_${fieldName}`, prefix.subarray(base + relStart, base + relEnd));
  }
}

emitSectionFields("section_text", 176);

{
  const [segmentName, base] = linkeditSegmentCommand;
  process.stdout.write(`\n# prefix region: ${segmentName}\n`);
  for (const [fieldName, relStart, relEnd] of segmentFields) {
    emitField(`${segmentName}_${fieldName}`, prefix.subarray(base + relStart, base + relEnd));
  }
}

for (const [commandName, base] of linkeditDataCommands) {
  emitLinkeditDataCommand(commandName, base);
}

emitSymtabCommand("lc_symtab", 360);
emitFields("lc_dysymtab", 384, dysymtabFields);
emitFields("lc_load_dylinker", 464, [
  ...stringCommandFields,
  ["name", 12, 24],
]);
emitFields("lc_uuid", 488, uuidFields);
emitFields("lc_build_version", 512, buildVersionFields);
emitFields("lc_source_version", 536, sourceVersionFields);
emitFields("lc_main", 552, mainFields);
emitFields("lc_load_dylib", 576, dylibFields);
for (const [commandName, base] of trailingLinkeditDataCommands) {
  emitLinkeditDataCommand(commandName, base);
}

for (const [name, start, end] of regions) {
  if (prefix.length < end) {
    process.stderr.write(`prefix is too short for ${name}: ${prefix.length} < ${end}\n`);
    process.exit(1);
  }
  if (name === "load_command_padding") {
    emitPadding(name, start, end);
  } else {
    emitRegion(name, prefix.subarray(start, end));
  }
}
