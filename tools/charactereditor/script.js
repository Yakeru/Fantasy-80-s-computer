const palette = ["#000000","#FFFFFF"]
const editor_width_px=8;
const editor_height_px=8;
const log = true;

var drawingMatrix = new Array(editor_width_px);
for (var i = 0; i < editor_width_px; i++) {
  drawingMatrix[i] = new Array(editor_height_px);
}

function initDrawingMatrix(paletteIndex) {
  for (var x = 0; x < editor_width_px; x++) {
    for (var y = 0; y < editor_height_px; y++) {
      drawingMatrix[x][y] = paletteIndex;
    }
  }
}

let spriteCanvas;
let spriteContext;

let paletteCanvas;
let paletteContext;

var selectedColor = "#000000";
var selectedPaletteIndex = 0;

function init() {
  spriteCanvas = document.getElementById("sprite");
  spriteCanvas.addEventListener("mousedown", function(e){
    spriteMouseDown(e);
  });
  spriteCanvas.addEventListener("mousemove", function(e){
    spriteMouseMove(e);
  });
  spriteCanvas.addEventListener("mouseup", function(e){
    spriteMouseUp(e);
  });

  spriteCanvas.addEventListener("mouseout", function(e){
    spriteMouseOut(e);
  });

  spriteContext = spriteCanvas.getContext("2d");

  paletteCanvas = document.getElementById("palette");
  paletteCanvas.addEventListener("mousedown", function(e){
    paletteClick(e);
  });
  paletteContext = paletteCanvas.getContext("2d");

  initDrawingMatrix(selectedPaletteIndex);
  drawSpriteGrid(selectedColor);
  drawColorPalette();

  document.getElementById("fill_button").addEventListener("click", fill);
  document.getElementById("generate").addEventListener("click", generate);
  document.getElementById("copy").addEventListener("click", copyToClipboard);
}

//Draw black squares and white borders
//to show an empty pixel grid
function drawSpriteGrid(color) {
  let spriteWidth = spriteContext.canvas.clientWidth;
  let spriteHeight = spriteContext.canvas.clientHeight;
  let square_width = spriteWidth/editor_width_px;
  let square_height = spriteHeight/editor_height_px;

  for(row_count = 0 ; row_count < editor_height_px ; row_count++) {
    for(column_count = 0 ; column_count < editor_width_px ; column_count++) {
      let square_x = square_width*column_count;
      let square_y = square_height*row_count;
      spriteContext.fillStyle = color;
      if(color == "#000000")
        spriteContext.strokeStyle = "White";
      else
        spriteContext.strokeStyle = "Black";
      spriteContext.fillRect(square_x, square_y, square_width, square_height);
      spriteContext.strokeRect(square_x, square_y, square_width, square_height);
    }
  }
}

function drawColorPalette() {
  let paletteWidth = paletteContext.canvas.clientWidth;
  let paletteHeight = paletteContext.canvas.clientHeight;
  let square_width = paletteWidth/palette.length;
  let square_height = paletteHeight;

  for(index=0 ; index < palette.length ; index++) {
      let square_x = square_width*index;
      let square_y = 0;
      paletteContext.fillStyle = palette[index];
      paletteContext.fillRect(square_x, square_y, square_width, square_height);
  }
}

var isDrawing = false;

function spriteMouseDown(event) {
  if(log) console.log("Sprite click !");
  isDrawing = true;
  draw();
}

function spriteMouseMove(event) {
  //Curseur coloré
  let rect = spriteCanvas.getBoundingClientRect();
  let x = event.clientX - rect.left;
  let y = event.clientY - rect.top;
  spriteContext.fillStyle = selectedColor;
  if(selectedColor == "#000000") {
    spriteContext.strokeStyle = "White";
  } else {
    spriteContext.strokeStyle = "Black";
  }
  paletteContext.beginPath();
  paletteContext.arc(50, 300, 15, 0, 2 * Math.PI, false); // full circle
  paletteContext.fill();
  paletteContext.stroke();

  if(isDrawing) draw();
}

function spriteMouseUp(event) {
  isDrawing = false;
}

function spriteMouseOut(event) {
  isDrawing = false;
}

function draw(){
  let rect = spriteCanvas.getBoundingClientRect();
  let x = event.clientX - rect.left;
  let y = event.clientY - rect.top;

  let spriteWidth = spriteContext.canvas.clientWidth;
  let spriteHeight = spriteContext.canvas.clientHeight;
  let square_width = spriteWidth/editor_width_px;
  let square_height = spriteHeight/editor_height_px;

  for(row_count = 0 ; row_count < editor_height_px ; row_count++) {
    for(column_count = 0 ; column_count < editor_width_px ; column_count++) {
      let square_x = square_width*column_count;
      let square_y = square_height*row_count;

      spriteContext.fillStyle = selectedColor;
      if(selectedColor == "#000000") {
        spriteContext.strokeStyle = "White";
      } else {
        spriteContext.strokeStyle = "Black";
      }

      if(x > square_x
        && x < square_x + square_width
        && y > square_y
        && y < square_y + square_height) {
          if(log) console.log("    Column : " + column_count + " , Row : " + row_count);
          drawingMatrix[column_count][row_count] = selectedPaletteIndex;
          spriteContext.clearRect(square_x, square_y, square_width, square_height);
          spriteContext.fillRect(square_x, square_y, square_width, square_height);
          spriteContext.strokeRect(square_x, square_y, square_width, square_height);
      }
    }
  }
}

function paletteClick(event) {
  if(log) console.log("Palette click !");
  selectedColor = "#000000";
  selectedPaletteIndex = 0;
  let rect = paletteCanvas.getBoundingClientRect();
  let x = event.clientX - rect.left;
  let y = event.clientY - rect.top;

  var imgData = paletteContext.getImageData(x,y,1,1).data;
  var hexCode = "#" + ("000000" + rgbToHex(imgData[0], imgData[1], imgData[2])).slice(-6);
  hexCode = hexCode.toUpperCase();
  if(log) console.log("    color: " + hexCode);

  for(index=0 ; index < palette.length ; index++) {
    if(palette[index] == hexCode) {
      selectedColor = palette[index];
      selectedPaletteIndex = index;
      if(log) console.log("    palette index: " + selectedPaletteIndex);
    }
  }
}

function fill() {
  if(log) console.log("Fill click !");
  initDrawingMatrix(0);
  drawSpriteGrid(palette[0]);
}

//*************************************************************************
//                          TOOLS
//*************************************************************************

function rgbToHex(r, g, b) {
  if (r > 255 || g > 255 || b > 255)
    throw "Invalid color component";
  return ((r << 16) | (g << 8) | b).toString(16);
}

//Generate Rust array from drawing

function generate() {
  var newHex = printDrawingMatrixTuRustArray();
  document.getElementById("rustArray").value += newHex + "\n";
}

function printDrawingMatrixTuRustArray() {
  let text = "'' => [";

  for (var y = 0; y < editor_height_px; y++) {
    var byte = "";
      for (var x = 0; x < editor_width_px; x++) {
        byte += drawingMatrix[x][y];
      }
      text += "0x" + binaryToHex(byte).result + ", "
  }

  text = text.slice(0, text.length - 3);
  text += "],";

  if(log) console.log(text);
  return text;
}

function copyToClipboard() {
  var textArea = document.getElementById("rustArray");
  textArea.select();
  textArea.setSelectionRange(0, 99999);
  navigator.clipboard.writeText(textArea.value);
}

// converts binary string to a hexadecimal string
// returns an object with key 'valid' to a boolean value, indicating
// if the string is a valid binary string.
// If 'valid' is true, the converted hex string can be obtained by
// the 'result' key of the returned object
function binaryToHex(s) {
  var i, k, part, accum, ret = '';
  for (i = s.length-1; i >= 3; i -= 4) {
      // extract out in substrings of 4 and convert to hex
      part = s.substr(i+1-4, 4);
      accum = 0;
      for (k = 0; k < 4; k += 1) {
          if (part[k] !== '0' && part[k] !== '1') {
              // invalid character
              return { valid: false };
          }
          // compute the length 4 substring
          accum = accum * 2 + parseInt(part[k], 10);
      }
      if (accum >= 10) {
          // 'A' to 'F'
          ret = String.fromCharCode(accum - 10 + 'A'.charCodeAt(0)) + ret;
      } else {
          // '0' to '9'
          ret = String(accum) + ret;
      }
  }
  // remaining characters, i = 0, 1, or 2
  if (i >= 0) {
      accum = 0;
      // convert from front
      for (k = 0; k <= i; k += 1) {
          if (s[k] !== '0' && s[k] !== '1') {
              return { valid: false };
          }
          accum = accum * 2 + parseInt(s[k], 10);
      }
      // 3 bits, value cannot exceed 2^3 - 1 = 7, just convert
      ret = String(accum) + ret;
  }
  return { valid: true, result: ret };
}
