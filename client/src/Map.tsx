import {Sprite, Text} from '@pixi/react';
import { TextStyle } from 'pixi.js'
import {Vector2} from './Common'

//Helper class to make it so any class that extends display base doesn't need to have a million things passed in
//through their constructor
export class DisplayArguments {
    axialCoordinates : Vector2;
    screenCoordinates : Vector2;
    imageName : string;

    constructor(axialCoordinates : Vector2, screenCoordinates : Vector2, imageName : string){
        this.axialCoordinates = axialCoordinates;
        this.screenCoordinates = screenCoordinates;
        this.imageName = imageName;
    }
}

//Base class for all things that needs to be displayed with a sprite and text over it (development only)
class DisplayBase{
    displayArguments : DisplayArguments;

    constructor(displayArguments : DisplayArguments){
        this.displayArguments = displayArguments;
    }

    getSprite() : JSX.Element {
        return(
            <Sprite
            image={"/" + this.displayArguments.imageName}
            scale={{ x: 0.5, y: 0.5 }}
            anchor={0.5}
            x={this.displayArguments.screenCoordinates.x}
            y={this.displayArguments.screenCoordinates.y}
          />
        );
    }

    //Override in other classes by redeclaring to change the text returned by the getText() function
    getTextString() : string {
        return "(" + this.displayArguments.axialCoordinates.x + ", " + this.displayArguments.axialCoordinates.y + ")";
    }

    getText() : JSX.Element {
        return(
            <Text
            text= {this.getTextString()}
            anchor={0.5}
            x={this.displayArguments.screenCoordinates.x}
            y={this.displayArguments.screenCoordinates.y}
            style={
              new TextStyle({
                align: 'center',
                fontFamily: '"Source Sans Pro", Helvetica, sans-serif',
                fontSize: 10,
              })
            }
          />
        );
    }
}

export enum ResourceType{
    WOOD,
    BRICK,
    WHEAT,
    SHEEP,
    ORE,
    ANY,
    NONE
}

export enum SettlementLevel{
    NONE,
    CITY,
    BASIC
}

export class PortData{
    input : ResourceType;
    numInputs : number;
    constructor(input : ResourceType, numInputs : number){
        this.input = input;
        this.numInputs = numInputs;
    }
}

export class SettlementData{
    playerId : number;
    settlementLevel : SettlementLevel;
    constructor(playerId : number, settlementLevel : SettlementLevel){
        this.playerId = playerId;
        this.settlementLevel = settlementLevel;
    }
}

export class Hex extends DisplayBase{
    resourceType : ResourceType;
    diceNumber : number;
    hasRobber : boolean;
    constructor(displayArguments : DisplayArguments, resourceType : ResourceType, diceNumber : number, hasRobber : boolean){
        super(displayArguments);
        this.resourceType = resourceType;
        this.diceNumber = diceNumber;
        this.hasRobber = hasRobber;
    }

    getTextString(): string {
        return super.getTextString() + " " + this.diceNumber;
    }
}

//Wrapper around the vertex class, used because some vertices will have the same axial coordinates, 
//and therefore same array index
export class GridVertex {
    top : Vertex | null;
    bottom : Vertex | null;
    constructor(top : Vertex | null, bottom : Vertex | null){
        this.top = top;
        this.bottom = bottom;
    }
}

export class Vertex extends DisplayBase {
    settlementData : SettlementData;
    portData : PortData;
    i : number;
    centerAxial : Vector2;
    constructor(displayArguments : DisplayArguments, settlementData : SettlementData, portData : PortData, i : number, centerAxial : Vector2){
        super(displayArguments);
        this.settlementData = settlementData;
        this.portData = portData;
        this.i = i;
        this.centerAxial = centerAxial;
    }
}