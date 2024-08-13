import {Sprite, Text} from '@pixi/react';
import { TextStyle } from 'pixi.js'
import {Vector3} from './Math'

//Helper class to make it so any class that extends display base doesn't need to have a million things passed in
//through their constructor
export class DisplayArguments {
    position : Vector3;
    imageName : string;
    rotation : number;

    constructor(position : Vector3, imageName : string, rotation : number = 0){
        this.position = position;
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
        return(
            <Sprite
            image={"/" + this.displayArguments.imageName}
            scale={{ x: 0.5, y: 0.5 }}
            anchor={0.5}
            rotation={this.displayArguments.rotation}
            x={this.displayArguments.position.x}
            y={this.displayArguments.position.y}
          />
        );
    }

    //Override in other classes by redeclaring to change the text returned by the getText() function
    getTextString() : string {
        return "(" + this.displayArguments.position.x + ", " + this.displayArguments.position.y + ")";
    }

    getText() : JSX.Element {
        return(
            <Text
            text= {this.getTextString()}
            anchor={0.5}
            x={this.displayArguments.position.x}
            y={this.displayArguments.position.y}
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

export class Vertex extends DisplayBase {
    settlementData : SettlementData;
    portData : PortData;

    constructor(displayArguments : DisplayArguments, settlementData : SettlementData, portData : PortData){
        super(displayArguments);
        this.settlementData = settlementData;
        this.portData = portData;
    }

    getTextString(): string {
        return "*";
    }
}