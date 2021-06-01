const encoder = new TextEncoder();
const decoder = new TextDecoder("utf-8");

const binary = await Deno.readFile("src/source.txt");
const source = decoder.decode(binary);
let data = source
    .split("\n\n")
    .map(matrix => matrix.split("\n").map(row => `        ${row}`).join('],\n'))
    .join('],\n    ],\n    [\n')
    .replaceAll("        ", "        [")
    .replaceAll("#", "true,  ")
    .replaceAll("-", "false, ");
data = `pub const DATA: [[[bool; 101]; 44]; 4383] = [\n    [\n${data}]\n    ]\n];`

await Deno.writeFile("src/data.rs", encoder.encode(data));