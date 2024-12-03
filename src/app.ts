import "../styles/style.scss";
import "../styles/style2.scss"

import { Application, Assets, Graphics, GraphicsContext} from "pixi.js";
//import myIcon from "../public/assets/Hexagon.svg";

(async () => {
    const app : Application = new Application();
    const canvas : HTMLElement | null = document.getElementById("canvas");
    if(!canvas){
        console.error("Error, canvas div not found!");
        return;
    }

    await app.init({
        resizeTo: canvas,
        backgroundAlpha: 0.9,
        antialias: true,
        resolution: window.devicePixelRatio || 1,
        autoDensity: true
    });

    const svg : GraphicsContext = await Assets.load({
        src: "./assets/Hexagon.svg",
        data: {
            parseAsGraphicsContext: true
        }
    });

    const rectangle : Graphics = new Graphics(svg)
    .fill({
        color: 0xffff
    });

    rectangle.scale.set(0.1, 0.1);
    rectangle.pivot.set(100, 100); 
    rectangle.position.set(300, 200);
    
    app.stage.addChild(rectangle);
    canvas.appendChild(app.canvas);

    app.ticker.add(() =>
        {
            rectangle.rotation += 0.01;
            rectangle.scale.set(2 + Math.sin(rectangle.rotation));
        });
})();