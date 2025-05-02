# `slipwayhq.svg`

A [Slipway](https://slipway.co/) Component which takes an SVG string and outputs the rendered SVG as
[a canvas](https://slipway.co/docs/guides/canvases).

This Component is used by Components such as [`slipwayhq.jsx`](https://github.com/slipwayhq/slipway_jsx)
and [`slipwayhq.echarts`](https://github.com/slipwayhq/slipway_echarts).

## Suggested Permissions

### `--allow-fonts`

This component may need to access fonts from the local system if it needs to render any text.

## Example Usage

Test the component by running the following command and pasting in the input when prompted:
```
slipway run-component "slipwayhq.svg.0.5.0" --allow-fonts
```

Input:
```json
{
  "width": 400,
  "height": 300,
  "svg": "<svg width=\"400\" height=\"300\" viewBox=\"0 0 400 300\" xmlns=\"http://www.w3.org/2000/svg\"><mask id=\"satori_om-id\"><rect x=\"0\" y=\"0\" width=\"400\" height=\"300\" fill=\"#fff\"/></mask><rect x=\"0\" y=\"0\" width=\"400\" height=\"300\" fill=\"#fff\"/><clipPath id=\"satori_cp-id-0\"><rect x=\"163\" y=\"79\" width=\"75\" height=\"65\"/></clipPath><mask id=\"satori_om-id-0\"><rect x=\"163\" y=\"79\" width=\"75\" height=\"65\" fill=\"#fff\"/></mask><image x=\"163\" y=\"79\" width=\"75\" height=\"65\" href=\"data:image/svg+xml;utf8,%3Csvg  fill=%22%23000%22 xmlns=%22http://www.w3.org/2000/svg%22 width=%22NaN%22 height=%22null%22 viewBox=%220 0 75 65%22%3E%3Cpath d=%22M37.59.25l36.95 64H.64l36.95-64z%22%3E%3C/path%3E%3C/svg%3E\" preserveAspectRatio=\"none\" clip-path=\"url(#satori_cp-id-0)\" mask=\"url(#satori_om-id-0)\"/><mask id=\"satori_om-id-0-0\"><rect x=\"163\" y=\"79\" width=\"0\" height=\"65\" fill=\"#fff\"/></mask><mask id=\"satori_om-id-1\"><rect x=\"114\" y=\"184\" width=\"172\" height=\"37\" fill=\"#fff\"/></mask><text x=\"114\" y=\"213.6875\" width=\"73.609375\" height=\"37.5\" font-weight=\"600\" font-style=\"normal\" font-size=\"32\" font-family=\"serif\" fill=\"black\">Hello</text><text x=\"187.609375\" y=\"213.6875\" width=\"6.296875\" height=\"37.5\" font-weight=\"600\" font-style=\"normal\" font-size=\"32\" font-family=\"serif\" fill=\"black\">,</text><text x=\"193.90625\" y=\"213.6875\" width=\"7.9375\" height=\"37.5\" font-weight=\"600\" font-style=\"normal\" font-size=\"32\" font-family=\"serif\" fill=\"black\"> </text><text x=\"201.84375\" y=\"213.6875\" width=\"83.3125\" height=\"37.5\" font-weight=\"600\" font-style=\"normal\" font-size=\"32\" font-family=\"serif\" fill=\"black\">World</text></svg>"
}
```

Output:
```json
{
  "canvas": {
    "width": 400,
    "height": 300,
    "data": "<encoded_rgba_bytes_omitted>"
  }
}
```