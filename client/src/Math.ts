export class Vector3{
    x : number;
    y : number;
    z : number;

    constructor(x : number, y : number, z : number = 1){
        this.x = x;
        this.y = y;
        this.z = z;
    }

    static Multiply(a : Vector3, b : number) : Vector3 {
        return new Vector3(a.x * b, a.y * b, a.z * b);
    }
}