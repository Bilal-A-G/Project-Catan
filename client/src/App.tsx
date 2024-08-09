import { Stage} from '@pixi/react';
import { width, height, gridSize } from './Constants';
import { HexAxialToIndex, ArraySizeFromGridSize, RollDice, HexAxialToScreen, GetCorner, VertexScreenToAxial, VertexAxialToIndex, HexAxialRound, HexScreenToAxial} from './Common';
import {Hex, ResourceType, DisplayArguments, GridVertex, Vertex, SettlementData, SettlementLevel, PortData} from "./Map";
import { Vector2 } from './Math';

let hexGrid : Hex[][] = new Array(ArraySizeFromGridSize(gridSize));
let vertexGrid : GridVertex[][] = new Array(ArraySizeFromGridSize(gridSize) + 4);
let spacing : Vector2 = new Vector2(54, 60);
let offset : Vector2 = new Vector2(width/2, height/2);

const InitializeHexGrid = () : Hex[][] => {
  //Initialize hex grid
  for(let i = 0; i < hexGrid.length; i++){
    hexGrid[i] = new Array(hexGrid.length);
  }

  //Initialized vertex grid
  for(let i = 0; i < vertexGrid.length; i++){
    vertexGrid[i] = new Array(vertexGrid.length);
  }

  for (let r : number = -gridSize; r < gridSize + 1; r++){
    let startIndex : number = -Math.min(r + gridSize, gridSize);
    let endIndex : number = Math.min(-(r - gridSize), gridSize);
    let rIndex : number = HexAxialToIndex(r, gridSize);

    for (let q : number = startIndex; q < endIndex + 1; q++){
      let qIndex : number = HexAxialToIndex(q, gridSize);
      let diceRoll : number = RollDice();
      let axial = new Vector2(q, r);
      let screen = HexAxialToScreen(axial, spacing, offset);
      let displayArguments = new DisplayArguments(axial, screen, "Hex.svg");

      //Create 6 vertices per hex
      for(let i = 0; i < 4; i++){
        let screenPosition : Vector2 = GetCorner(screen, i);
        //let vertexAxial : Vector2 = VertexScreenToAxial(screenPosition, spacing, offset, i);
        //let vertexQIndex : number = VertexAxialToIndex(vertexAxial.x, gridSize);
        //let vertexRIndex : number = VertexAxialToIndex(vertexAxial.y, gridSize);
        //let vertexDisplayArguments = new DisplayArguments(vertexAxial, screenPosition, "Circle.svg");
        //let settlementData = new SettlementData(0, SettlementLevel.NONE);
        //let portData = new PortData(ResourceType.NONE, 0);

        //let vertex : Vertex = new Vertex(vertexDisplayArguments, settlementData, portData, i, axial);
        //console.log(vertexAxial);
        //console.log(axial);
        //Every other vertex is west, first one is east
        //vertexGrid[vertexRIndex][vertexQIndex]= new GridVertex(vertex, vertex);
      }

      hexGrid[rIndex][qIndex] = new Hex(
        displayArguments,
        ResourceType.NONE,
        diceRoll,
        false
      );
    }
  }

  return hexGrid;
}

const onMouseMoved = (e : React.MouseEvent<HTMLCanvasElement, MouseEvent>) => {
  let hexRounded : Vector2 = HexAxialRound(HexScreenToAxial(new Vector2(e.screenX, e.screenY), spacing, offset));
  console.log(hexRounded);
}

const App = () => {
  InitializeHexGrid();
  const sprites : JSX.Element[] = [];
  const texts : JSX.Element[] = [];

  hexGrid.forEach((row : Hex[] | undefined) => {
      if (row) {
          row.forEach((hex : Hex | undefined) => {
              if (hex) {
                  sprites.push(hex.getSprite());
                  texts.push(hex.getText());
              }
          });
      }
  });

  vertexGrid.forEach((row : GridVertex[] | undefined) => {
    if(row){
      row.forEach((gridVertex : GridVertex | undefined) => {
        if(gridVertex){
          if(gridVertex.east){
            console.log("Rendering east");
            sprites.push(gridVertex.east.getSprite());
            texts.push(gridVertex.east.getText());
          }
          if(gridVertex.west){
            console.log("Rendering west");
            sprites.push(gridVertex.west.getSprite());
            texts.push(gridVertex.west.getText());
          }
        }
      })
    }
  })

  return (
    <div className='h-screen w-screen content-center'>
      <div className='flex justify-center'>
        <Stage width={width} height={height} options={{ background: 0xffffff }} onMouseMove={(e) => {onMouseMoved(e)}}>
            {sprites}
            {texts}
        </Stage>
      </div>
    </div>
  );
};

export default App;