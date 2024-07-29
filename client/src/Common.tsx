//Convert one of your q or r values to it's corresponding grid index
export function CoordinateToIndex(coordinate : number, gridSize : number) : number {
    return coordinate + gridSize;
}
  
  //Convert one of your grid indices into it's corresponding q or r value
export function IndexToCoordinate(index : number, gridSize : number) : number {
    return index - gridSize;
}
  
//Convert the grid size to the size of an array dimension
export function ArraySizeFromGridSize(gridSize : number) : number{
    return gridSize * 2 + 1;
}

//Base class for all things that needs to be displayed with a sprite and text over it (development only)
class DisplayBase{
    sprite : JSX.Element;
    text : JSX.Element;

    constructor(sprite : JSX.Element, text : JSX.Element){
        this.sprite = sprite;
        this.text = text;
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

export class Hex extends DisplayBase{
    resourceType : ResourceType;
    constructor(sprite : JSX.Element, text : JSX.Element, resourceType : ResourceType){
        super(sprite, text);
        this.resourceType = resourceType;
    }
}