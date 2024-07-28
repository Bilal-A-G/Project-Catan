class Hex 
{
    constructor(sprite, text)
    {
        this.sprite = sprite;
        this.text = text;
    }
}

import { Stage, Sprite, Text} from '@pixi/react';
import { TextStyle } from 'pixi.js'

//Convert one of your q or r values to it's corresponding grid index
export function CoordinateToIndex(coordinate, gridSize) {
  return coordinate + gridSize;
}

//Convert one of your grid indices into it's corresponding q or r value
export function IndexToCoordinate(index, gridSize) {
  return index - gridSize;
}

//Convert the grid size to the size of an array dimension
export function ArraySizeFromGridSize(gridSize){
  return gridSize * 2 + 1;
}

const DrawMap = (gridSize, xOffset, yOffset) => {
  var gridSpacing = 60;
  var horizSpacing = 54;
  let hexGrid = new Array(ArraySizeFromGridSize(gridSize));

  for (let r = -gridSize; r < gridSize + 1; r++){
    let startIndex = -Math.min(r + gridSize, gridSize);
    let endIndex = Math.min(-(r - gridSize), gridSize);

    let rIndex = CoordinateToIndex(r, gridSize);
    hexGrid[rIndex] = new Array(ArraySizeFromGridSize(gridSize));

    for (let q = startIndex; q < endIndex + 1; q++){
      let qIndex = CoordinateToIndex(q, gridSize);
      hexGrid[rIndex][qIndex] = new Hex(
        <Sprite
          image={"/Hex.svg"}
          scale={{ x: 0.5, y: 0.5 }}
          anchor={0.5}
          x={xOffset + q * horizSpacing}
          y={yOffset + (r + q/2) * gridSpacing}
        />,
        <Text
          text= {"(" + q + ", " + r + ")"}
          anchor={0.5}
          x={xOffset + q * horizSpacing}
          y={yOffset + (r + q/2) * gridSpacing}
          style={
            new TextStyle({
              align: 'center',
              fontFamily: '"Source Sans Pro", Helvetica, sans-serif',
              fontSize: 10,
              fontWeight: '10',
            })
          }
        />
      );
    }
  }

  return hexGrid;
}

const App = () => {
  let width = 800;
  let height = 600;
  const hexes = DrawMap(2, width/2, height/2);
  const sprites = [];
  const texts = [];

  hexes.forEach(row => {
      if (row) {
          row.forEach(hex => {
              if (hex) {
                  sprites.push(hex.sprite);
                  texts.push(hex.text);
              }
          });
      }
  });

  return (
    <div className='h-screen w-screen content-center'>
      <div className='flex justify-center'>
        <Stage width={width} height={height} options={{ background: 0x1e1e1e }}>
              {sprites}
              {texts}
        </Stage>
      </div>
    </div>
  );
};

export default App;