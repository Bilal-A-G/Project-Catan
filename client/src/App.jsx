import { Stage, Sprite} from '@pixi/react';

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

  for (let q = -gridSize; q < gridSize + 1; q++){
    let startIndex = -Math.min(q + gridSize, gridSize);
    let endIndex = Math.min(-(q - gridSize), gridSize);

    let qIndex = CoordinateToIndex(q, gridSize);
    hexGrid[qIndex] = new Array(ArraySizeFromGridSize(gridSize));

    for (let r = startIndex; r < endIndex + 1; r++){
      let rIndex = CoordinateToIndex(r, gridSize);
      hexGrid[qIndex][rIndex] = (
        <Sprite
        image={"/Hex.svg"}
        scale={{ x: 0.5, y: 0.5 }}
        anchor={0.5}
        x={xOffset + r * horizSpacing}
        y={yOffset + (q + r/2) * gridSpacing}
        />
      );
    }
  }

  return hexGrid;
}

const App = () => {
  let width = 800;
  let height = 600;
  return (
    <div className='h-screen w-screen content-center'>
      <div className='flex justify-center'>
        <Stage width={width} height={height} options={{ background: 0x1e1e1e }}>
          {DrawMap(2, width/2, height/2)}
        </Stage>
      </div>
    </div>
  );
};

export default App;