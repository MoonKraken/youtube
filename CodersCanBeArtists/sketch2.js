let X_MAX = 700;
let Y_MAX = 700;

let colorOrigins = [
  {
    x: 0,
    y: 0,
    r: 255,
    g: 0,
    b: 0
  },
  {
    x: 700,
    y: 700,
    r: 0,
    g: 255,
    b: 0
  },
  {
    x: 0,
    y: 700,
    r: 0,
    g: 0,
    b: 255
  },
  {
    x: 700,
    y: 0,
    r: 0,
    g: 0,
    b: 0
  }
];

function setup() {
  createCanvas(X_MAX, Y_MAX);
  angleMode(DEGREES);
}



function draw() {
  randomSeed(6);
  background(255);
  noStroke();
  squareGrid(20, -100, -100, X_MAX + 100, Y_MAX + 100);
}

function squareGrid(squareSize, startX, startY, endX, endY) {
    
    for(let x = startX; x < endX; x+=squareSize) {
        for(let y = startY; y < endY; y+=squareSize) {
            let color = determineColor(x, y);
            fill(color.r, color.g, color.b);
            square(x, y, squareSize);
        }
    }
}

function determineColor(x, y) {
  // determine total distance of all color origins
  var totalDistances = 0;
  var distances = [];
  for (let i=0; i < colorOrigins.length; ++i) {
    let currentOrigin = colorOrigins[i];
    let distance = euclideanDistance(x,y,currentOrigin.x, currentOrigin.y);
    totalDistances += distance;
    distances.push(distance);
  }

  var r = 0;
  var g = 0;
  var b = 0;

  for (let i=0; i < colorOrigins.length; ++i) {
    let currentOrigin = colorOrigins[i];
    // use inverse distance percentage to determine affect of origin on this color
    let percentageImpact = 1 - distances[i] / totalDistances;
    r += currentOrigin.r * percentageImpact;
    g += currentOrigin.g * percentageImpact;
    b += currentOrigin.b * percentageImpact;
  }

  return {
    r,
    g,
    b
  };
}

function euclideanDistance(x1, y1, x2, y2) {
  return Math.sqrt(Math.pow((x1 - x2), 2) + Math.pow((y1 - y2), 2));
}
