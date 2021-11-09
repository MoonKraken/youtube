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
    b: 150
  },
  {
    x: 700,
    y: 0,
    r: 0,
    g: 3,
    b: 1
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
  for (let i=0; i < 200; ++i) {
    let fn = random([ellipse, rect]);
    drawShape(200, 200, 20, 20, fn);
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

function drawShape(minWidth, minHeight, widthVariance, heightVariance, fn) {
  let xpos = random(X_MAX);
  let ypos = random(Y_MAX);
  let width = minWidth + random(widthVariance);
  let height = minHeight + random(heightVariance);

  // interpolate between color points
  let color = determineColor(xpos, ypos);

  fill(color.r, color.g, color.b);
  rotate(random(360));
  fn(xpos, ypos, width, height);
}
