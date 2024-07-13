import { Stage, Sprite} from '@pixi/react';

const App = () => {
  return (
    <div className='h-screen w-screen content-center'>
      <div className='flex justify-center'>
        <Stage width={800} height={600} options={{ background: 0x1e1e1e }}>
          <Sprite
            image={"/react.svg"}
            scale={{ x: 0.5, y: 0.5 }}
            anchor={0.5}
            x={150}
            y={150}
          />
        </Stage>
      </div>
    </div>
  );
};

export default App;