import { Stage} from '@pixi/react';
import { width, height, gridSize, CalulateSCToDC} from './Constants';
import { HexAxialToIndex, ArraySizeFromGridSize, RollDice, HexAxialToDC, GetCorner, VertexScreenToAxial, VertexAxialToIndex, HexAxialRound, HexDCToAxial} from './Common';
import {Hex, ResourceType, DisplayArguments, GridVertex, Vertex, SettlementData, SettlementLevel, PortData} from "./Map";
import {Matrix3x3, Vector3 } from './Math';

let hexGrid : Hex[][] = new Array(ArraySizeFromGridSize(gridSize));
let vertexGrid : GridVertex[][] = new Array(ArraySizeFromGridSize(gridSize) + 4);

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
      let axial = new Vector3(q, r);
      let dc = HexAxialToDC(axial);
      let displayArguments = new DisplayArguments(axial, dc, "Hex.svg");
      
      //Create 6 vertices per hex
      for(let i = 0; i < 4; i++){
        let screenPosition : Vector3 = GetCorner(dc, i);
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
  let matrix : Matrix3x3 = CalulateSCToDC();
  let NDCMouse : Vector3 = Matrix3x3.MultiplyVec(matrix, new Vector3(e.clientX, e.clientY));
  let inverseMatrix : Matrix3x3 | null = Matrix3x3.Inverse(matrix);
  if(inverseMatrix == null){
    return;
  }
  let screenMouse : Vector3 = Matrix3x3.MultiplyVec(inverseMatrix, NDCMouse);
}

const App = () => {
  InitializeHexGrid();
  const sprites : JSX.Element[] = [];
  const texts : JSX.Element[] = [];

  hexGrid.forEach((row : Hex[] | undefined) => {
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
  vertexGrid.forEach((row : GridVertex[] | undefined) => {
    if(row){
      row.forEach((gridVertex : GridVertex | undefined) => {
        if(gridVertex){
          if(gridVertex.east){
            console.log("Rendering east");
            let sprite : JSX.Element | null = gridVertex.east.getSprite();
            if(sprite != null){
              sprites.push(sprite);
            }
            texts.push(gridVertex.east.getText());
          }
          if(gridVertex.west){
            console.log("Rendering west");
            let sprite : JSX.Element | null = gridVertex.west.getSprite();
            if(sprite != null){
              sprites.push(sprite);
            }
            texts.push(gridVertex.west.getText());
          }
        }
      })
    }
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