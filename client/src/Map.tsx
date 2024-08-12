import {Sprite, Text} from '@pixi/react';
import { TextStyle } from 'pixi.js'
import {Matrix3x3, Vector3} from './Math'
import {DCToCC } from './Constants';

//Helper class to make it so any class that extends display base doesn't need to have a million things passed in
//through their constructor
export class DisplayArguments {
    axialCoordinates : Vector3;
    deviceCoordinates : Vector3;
    imageName : string;
    rotation : number;

    constructor(axialCoordinates : Vector3, deviceCoordinates : Vector3, imageName : string, rotation : number = 0){
        this.axialCoordinates = axialCoordinates;
        this.deviceCoordinates = deviceCoordinates;
        this.imageName = imageName;
        this.rotation = rotation;
    }
}

//Base class for all things that needs to be displayed with a sprite and text over it (development only)
class DisplayBase{
    displayArguments : DisplayArguments;

    constructor(displayArguments : DisplayArguments){
        this.displayArguments = displayArguments;
    }

    getSprite() : JSX.Element | null {
        let canvasCoordinates : Vector3 = Matrix3x3.MultiplyVec(DCToCC, this.displayArguments.deviceCoordinates);
        return(
            <Sprite
            image={"/" + this.displayArguments.imageName}
            scale={{ x: 0.5, y: 0.5 }}
            anchor={0.5}
            rotation={this.displayArguments.rotation}
            x={canvasCoordinates.x}
            y={canvasCoordinates.y}
          />
        );
    }

    //Override in other classes by redeclaring to change the text returned by the getText() function
    getTextString() : string {
        return "(" + this.displayArguments.axialCoordinates.x + ", " + this.displayArguments.axialCoordinates.y + ")";
    }

    getText() : JSX.Element {
        let canvasCoordinates : Vector3 = Matrix3x3.MultiplyVec(DCToCC, this.displayArguments.deviceCoordinates);
        return(
            <Text
            text= {this.getTextString()}
            anchor={0.5}
            x={canvasCoordinates.x}
            y={canvasCoordinates.y}
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
        return "*" ;
    }
}

//Wrapper around the vertex class, used because some vertices will have the same axial coordinates, 
//and therefore same array index
export class GridVertex {
    west : Vertex | null;
    east : Vertex | null;
    constructor(west : Vertex | null, east : Vertex | null){
        this.west = west;
        this.east = east;
    }
}

export class Vertex extends DisplayBase {
    settlementData : SettlementData;
    portData : PortData;
    i : number;
    centerAxial : Vector3;
    constructor(displayArguments : DisplayArguments, settlementData : SettlementData, portData : PortData, i : number, centerAxial : Vector3){
        super(displayArguments);
        this.settlementData = settlementData;
        this.portData = portData;
        this.i = i;
        this.centerAxial = centerAxial;
    }

    getTextString(): string {
        return "";
    }
}