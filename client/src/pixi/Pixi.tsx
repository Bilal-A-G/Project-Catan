import { useApp } from "@pixi/react";
import { Application, Container } from "pixi.js";

function run(app : any)
{
    const container : Container = new Container();
    app.stage.addChild(container);
}

export default function PixiApp()
{
    let app = useApp();
    app.stage.removeChildren();
    return (
    <></>
    )    
}