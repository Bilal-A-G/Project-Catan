import {describe, test, it, expect} from "vitest"
import {HexAxialToDC, HexScreenToAxial, RollDice} from "../src/Common";
import { Vector3 } from "../src/Math";

describe('Hex Axial to DC', () => {
    it('should return (400, 400) when axial = (0, 0) and offset = (400, 400)', () => {
        let screen : Vector3 = HexAxialToDC(new Vector3(0, 0));

        expect(screen).toStrictEqual(new Vector3(400, 400));
    })
})

describe('Hex Screen to Axial', () => {
    it('should return (0, 0) when given offset = (400, 400) and screen = (400, 400)', () => {
        let axial : Vector3 | null = HexScreenToAxial(new Vector3(400, 400));
        //console.log(axial);

        if(axial == null){
            return;
        }

        expect(axial.x).toBeCloseTo(0);
        expect(axial.y).toBeCloseTo(0);
    })
})

describe('Roll Dice', () => {
    it('should return a number between 1 and 12', () => {
        for(let i = 0; i < 10000; i++){
            let num = RollDice();

            expect(num).toBeGreaterThanOrEqual(1);
            expect(num).toBeLessThanOrEqual(12);
        }
    })
})

