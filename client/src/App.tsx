import { Stage, Sprite, Text} from '@pixi/react';
import { TextStyle } from 'pixi.js'
import { CoordinateToIndex, ArraySizeFromGridSize} from './Common';
import {Hex, ResourceType} from "./Common";

const InitializeHexGrid = (gridSize : number, xOffset : number, yOffset : number) : Hex[][] => {
  var gridSpacing : number = 60;
  var horizSpacing : number = 54;
  let hexGrid : Hex[][] = new Array(ArraySizeFromGridSize(gridSize));

  for (let r : number = -gridSize; r < gridSize + 1; r++){
    let startIndex : number = -Math.min(r + gridSize, gridSize);
    let endIndex : number = Math.min(-(r - gridSize), gridSize);

    let rIndex : number = CoordinateToIndex(r, gridSize);
    hexGrid[rIndex] = new Array(ArraySizeFromGridSize(gridSize));

    for (let q : number = startIndex; q < endIndex + 1; q++){
      let qIndex : number = CoordinateToIndex(q, gridSize);
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
            })
          }
        />,
        ResourceType.NONE
      );
    }
  }

  return hexGrid;
}

const App = () => {
  let width : number = 800;
  let height : number = 600;
  const hexes : Hex[][] = InitializeHexGrid(2, width/2, height/2);
  const sprites : JSX.Element[] = [];
  const texts : JSX.Element[] = [];

  hexes.forEach((row : Hex[] | undefined) => {
      if (row) {
          row.forEach((hex : Hex | undefined) => {
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