import { Stage} from '@pixi/react';
import { width, height, gridSize, hexSize} from './Constants';
import {RollDice, GetCorner} from './Common';
import {Hex, ResourceType, DisplayArguments, Vertex, SettlementData, SettlementLevel, PortData} from "./Map";
import {Vector3 } from './Math';

let hexGrid : Hex[] = new Array();
let vertexGrid : Vertex[] = new Array();

let offset : Vector3 = new Vector3(400, 300);

const InitializeHexGrid = () => {
  for (let r : number = -gridSize; r < gridSize + 1; r++){
    let startIndex : number = -Math.min(r + gridSize, gridSize);
    let endIndex : number = Math.min(-(r - gridSize), gridSize);

    for (let q : number = startIndex; q < endIndex + 1; q++){
      let diceRoll : number = RollDice();
      let hexPosition = new Vector3((q * 3/2 + r * Math.sqrt(3)/2) * hexSize + offset.x, (r * Math.sqrt(3) * hexSize) + offset.y);
      let displayArguments = new DisplayArguments(hexPosition, "Hex.svg");
      
      hexGrid.push(new Hex(
        displayArguments,
        ResourceType.NONE,
        diceRoll,
        false
      ));

      //Create 6 vertices per hex
      for(let i = 0; i < 6; i++){
        let vertexPosition : Vector3 = GetCorner(hexPosition, i);
        let vertexDisplayArguments = new DisplayArguments(vertexPosition, "Circle.svg");
        let settlementData = new SettlementData(0, SettlementLevel.NONE);
        let portData = new PortData(ResourceType.NONE, 0);

        let vertex : Vertex = new Vertex(vertexDisplayArguments, settlementData, portData);
        vertexGrid.push(vertex);
      }
    }
  }
}

const onMouseMoved = (e : React.MouseEvent<HTMLCanvasElement, MouseEvent>) => {
  console.log(e.screenX + ", " + e.screenY);
}

const App = () => {
  InitializeHexGrid();
  const sprites : JSX.Element[] = [];
  const texts : JSX.Element[] = [];

  hexGrid.forEach((hex : Hex) => {
    if (hex) {
      let sprite : JSX.Element | null = hex.getSprite();
      if (sprite != null){
        sprites.push(sprite);
      }
      texts.push(hex.getText());
  }
  });

  //TODO refactor, remove code duplication
  vertexGrid.forEach((vertex : Vertex) => {
    if(vertex){
      let sprite = vertex.getSprite();
      if(!sprite){
        return;
      }
      sprites.push(sprite);
      texts.push(vertex.getText());
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