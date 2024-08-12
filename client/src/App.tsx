import { Stage} from '@pixi/react';
import { width, height, gridSize, CalulateSCToDC} from './Constants';
import { HexAxialToIndex, ArraySizeFromGridSize, RollDice, HexAxialToDC, GetCorner, VertexScreenToAxial, VertexAxialToIndex, HexAxialRound, HexDCToAxial} from './Common';
import {Hex, ResourceType, DisplayArguments, GridVertex, Vertex, SettlementData, SettlementLevel, PortData} from "./Map";
import {Matrix3x3, Vector3 } from './Math';
import { Sprite } from 'pixi.js';

let hexGrid : Hex[][] = new Array(ArraySizeFromGridSize(gridSize));
let vertexGrid : GridVertex[][] = new Array(ArraySizeFromGridSize(gridSize) + 3);

const InitializeHexGrid = () => {
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
      let axial = new Vector3(q, r);
      let dc = HexAxialToDC(axial);
      let displayArguments = new DisplayArguments(axial, dc, "Hex.svg");
      
      hexGrid[rIndex][qIndex] = new Hex(
        displayArguments,
        ResourceType.NONE,
        diceRoll,
        false
      );

      let maxR : number = 2;
      let rStart : number = 0;
      for(let q : number = -1; q < 3; q++){
        maxR--;
        if(q == -1){
          maxR = 2;
        }
        if(q == 0){
          maxR = 2;
          rStart = -1;
        }
        if(q == 1){
          maxR = 2;
          rStart = -2;
        }
        for(let r : number = rStart; r < maxR; r++){
          let rBasis : Vector3 = Vector3.Multiply(new Vector3(0, Math.sqrt(3)), 35);
          let qBasis : Vector3 = Vector3.Multiply(new Vector3(Math.sqrt(3), 0), 31);
          let vertexDisplayArguments = new DisplayArguments(new Vector3(0, 0), new Vector3(-30 + qBasis.x * q + rBasis.x * r, -1.5 + qBasis.y * q + rBasis.y * r + q * 30), "Triangle.svg", Math.PI/2);
          let settlementData = new SettlementData(0, SettlementLevel.NONE);
          let portData = new PortData(ResourceType.NONE, 0);
          let vertex : Vertex = new Vertex(vertexDisplayArguments, settlementData, portData, 0, axial);
          vertexGrid[q + 1][r + 2] = new GridVertex(vertex, null);
        }
      }

      if(true){
        continue;
      }
      //Create 6 vertices per hex
      for(let i = 0; i < 6; i++){
        let screenPosition : Vector3 = GetCorner(dc, i);
        let vertexAxial : Vector3 | null = VertexScreenToAxial(screenPosition, i);
        if(vertexAxial == null){
          continue;
        }
        let vertexQIndex : number = VertexAxialToIndex(vertexAxial.x, gridSize);
        let vertexRIndex : number = VertexAxialToIndex(vertexAxial.y, gridSize);
        let vertexDisplayArguments = new DisplayArguments(vertexAxial, screenPosition, "Circle.svg");
        let settlementData = new SettlementData(0, SettlementLevel.NONE);
        let portData = new PortData(ResourceType.NONE, 0);

        let vertex : Vertex = new Vertex(vertexDisplayArguments, settlementData, portData, i, axial);
        //Every other vertex is west, first one is east
        console.log(vertexAxial.x + " " + vertexAxial.y + " " + (i % 2 == 0));
        vertexGrid[vertexRIndex][vertexQIndex] = new GridVertex(i % 2 == 0 ? null : vertex, i % 2 == 0 ? vertex : null);
      }
    }
  }
}

const onMouseMoved = (e : React.MouseEvent<HTMLCanvasElement, MouseEvent>) => {
  let matrix : Matrix3x3 = CalulateSCToDC();
  let DCMouse : Vector3 = Matrix3x3.MultiplyVec(matrix, new Vector3(e.clientX, e.clientY));
  let inverseMatrix : Matrix3x3 | null = Matrix3x3.Inverse(matrix);
  if(inverseMatrix == null){
    return;
  }
  let screenMouse : Vector3 = Matrix3x3.MultiplyVec(inverseMatrix, DCMouse);
  let hexDCToAxial : Vector3 | null = HexDCToAxial(DCMouse);
  if(hexDCToAxial == null){
    return;
  }
  let hexAxial : Vector3 = HexAxialRound(hexDCToAxial);
  //console.log(hexAxial);
}

const App = () => {
  InitializeHexGrid();
  const sprites : JSX.Element[] = [];
  const texts : JSX.Element[] = [];

  hexGrid.forEach((row : Hex[]) => {
      if (row) {
          row.forEach((hex : Hex | undefined) => {
              if (hex) {
                  let sprite : JSX.Element | null = hex.getSprite();
                  if (sprite != null){
                    sprites.push(sprite);
                  }
                  texts.push(hex.getText());
              }
          });
      }
  });

  //TODO refactor, remove code duplication
  vertexGrid.forEach((row : GridVertex[]) => {
    row.forEach((vertex : GridVertex | undefined) => {
      if(vertex){
        let sprite : JSX.Element | null = null;
        let text : JSX.Element | null = null;
        if(vertex.east){
          sprite = vertex.east.getSprite();
          text = vertex.east.getText();
        }
        else if (vertex.west){
          sprite = vertex.west.getSprite();
          text = vertex.west.getText();
        }    
        if(sprite && text){
          sprites.push(sprite);
          texts.push(text);
        }
      }
    })
  })

  return (
    <div className='h-screen w-screen content-center'>
      <div className='flex justify-center'>
        <Stage width={width} height={height} options={{ background: 0x00000 }} onMouseMove={(e) => {onMouseMoved(e)}}>
            {sprites}
            {texts}
        </Stage>
      </div>
    </div>
  );
};

export default App;