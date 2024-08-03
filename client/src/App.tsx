import { Stage} from '@pixi/react';
import { width, height, gridSize } from './Constants';
import { HexAxialToIndex, ArraySizeFromGridSize, RollDice, Vector2, HexAxialToScreen, GetCorner, VertexScreenToAxial, VertexAxialToIndex} from './Common';
import {Hex, ResourceType, DisplayArguments, GridVertex, Vertex, SettlementData, SettlementLevel, PortData} from "./Map";

let hexGrid : Hex[][] = new Array(ArraySizeFromGridSize(gridSize));
let vertexGrid : GridVertex[] = new Array(ArraySizeFromGridSize(gridSize) + 2);
let spacing : Vector2 = new Vector2(54, 60);
let offset : Vector2 = new Vector2(width/2, height/2);

const InitializeHexGrid = () : Hex[][] => {
  //Initialize hex grid
  for(let i = 0; i < hexGrid.length; i++){
    hexGrid[i] = new Array(hexGrid.length);
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
      for(let i = 0; i < 6; i++){
        let screenPosition : Vector2 = GetCorner(screen, i);
        let vertexAxial : Vector2 = VertexScreenToAxial(screenPosition, spacing, offset, i);
        let vertexQIndex : number = VertexAxialToIndex(vertexAxial.x, gridSize);
        let vertexRIndex : number = VertexAxialToIndex(vertexAxial.y, gridSize);
        let vertexDisplayArguments = new DisplayArguments(vertexAxial, screenPosition, "Circle.svg");
        let settlementData = new SettlementData(0, SettlementLevel.NONE);
        let portData = new PortData(ResourceType.NONE, 0);

        console.log(vertexAxial.x + ", " + vertexAxial.y + " " + axial.x + ", " + axial.y);
        vertexGrid.push(new GridVertex(new Vertex(vertexDisplayArguments, settlementData, portData, i, axial), null));
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

  vertexGrid.forEach((row : GridVertex | undefined) => {
    if(row){
      //row.forEach((gridVertex : GridVertex | undefined) => {
        //if(gridVertex){
          if(row.top){
            sprites.push(row.top.getSprite());
            texts.push(row.top.getText());
          }
          if(row.bottom){
            sprites.push(row.bottom.getSprite());
            texts.push(row.bottom.getText());
          }
        //}
      //})
    }
  })

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