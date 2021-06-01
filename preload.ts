const encoder = new TextEncoder();
const decoder = new TextDecoder("utf-8");

const binary = await Deno.readFile("src/source.txt");
const source = decoder.decode(binary);

getSize(source);
const [FRAMES, ROWS, COLS] = getSize(source);

let data = source
    .split("\n\n")
    .map(matrix => matrix.split("\n").map(row => `        ${row}`).join('],\n'))
    .join('],\n    ],\n    [\n')
    .replaceAll("        ", "        [")
    .replaceAll("#", "true,  ")
    .replaceAll("-", "false, ");
data = `pub const DATA: [[[bool; ${COLS}]; ${ROWS}]; ${FRAMES}] = [\n    [\n${data}]\n    ]\n];`

await Deno.writeFile("src/data.rs", encoder.encode(data));

function getSize(source: string) {
    const frames = source.split("\n\n");
    const framesLength = frames.length;
    const rows = frames[0].split('\n');
    const rowsLength = rows.length;
    const cols = rows[0];
    const colsLength = cols.length;
    return [framesLength, rowsLength, colsLength]
}