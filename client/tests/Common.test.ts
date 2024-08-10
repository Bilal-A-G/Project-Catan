import {describe, test, it, expect} from "vitest"
import {HexAxialToDC, HexDCToAxial, RollDice} from "../src/Common";
import { Vector3 } from "../src/Math";
import { hexSize } from "../src/Constants";

describe('Hex Axial to DC', () => {
    it('should return (0, hexSize * sqrt(3)) when axial = (0, 1)', () => {
        let dc : Vector3 = HexAxialToDC(new Vector3(0, 1));
        expect(dc).toStrictEqual(new Vector3(0, hexSize * Math.sqrt(3)));
    })
})

describe('Hex DC to Axial', () => {
    it('should return (0, 1) when DC = (0, hexSize * sqrt(3))', () => {
        let axial : Vector3 | null = HexDCToAxial(new Vector3(0, hexSize * Math.sqrt(3)));
        if(axial == null){
            return;
        }
        expect(axial.x).toBeCloseTo(0);
        expect(axial.y).toBeCloseTo(1);
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

