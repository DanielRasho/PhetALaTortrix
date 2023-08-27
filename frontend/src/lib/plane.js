/**
 * Draws a plane according to the given parameters.
 * @param {Object} two The renderer of the TwoJS
 * @param {Number} columns The number of columns in the graph
 * @param {Number} rows The number of rows in the graph
 * @param {String} guideLinesColor The color of the lines in the background
 */
export const initializePlane = (two, columns = 25, rows = 11, guideLinesColor = "#cccccc") => {
    // Draw columns
    let col_gap = two.width / columns;
    for (let i = 0; i < columns; i++) {
        let x = col_gap/2 + i * col_gap;
        let line = two.makeLine(x, 0, x, two.height);
        line.stroke = guideLinesColor;
    }

    // Draw rows
    let row_gap = two.height / rows;
    for (let i = 0; i < rows; i++) {
        let y = row_gap /2 + i*row_gap;
        let line = two.makeLine(0, y, two.width, y);
        line.stroke = guideLinesColor;
    }

    // Axes
    let yAxisPos = col_gap / 2 + 2*col_gap;
    let yAxis = two.makeArrow(yAxisPos, two.height, yAxisPos, 0, 12);
    yAxis.linewidth = 2;

    let xAxisPos = two.height - (rows * row_gap / 2);
    let xAxis = two.makeArrow(0, xAxisPos, two.width, xAxisPos, 12);
    xAxis.linewidth = 2;
}