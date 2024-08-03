import {describe, test, it, expect} from "vitest"
import {HexAxialToScreen, HexScreenToAxial, RollDice, Vector2} from "../src/Common";

describe('Hex Axial to Screen', () => {
    it('should return offset when given (0, 0) and (offset.x, offset.y - spacing.y) when given (0, -1)', () => {
        const spacing : Vector2 = new Vector2(Math.random() - 0.5 * 20 , Math.random() - 0.5 * 20);
        const offset : Vector2 = new Vector2(Math.random() - 0.5 * 200, Math.random() - 0.5 * 200);
        let screen : Vector2 = HexAxialToScreen(new Vector2(0, 0), spacing, offset);

        expect(screen).toStrictEqual(new Vector2(offset.x, offset.y));

        screen = HexAxialToScreen(new Vector2(0, -1), spacing, offset);

        expect(screen).toStrictEqual(new Vector2(offset.x, offset.y - spacing.y));
    })
})

describe('Hex Screen to Axial', () => {
    it('should return (0, 0) when given offset and (0, -1) when given (offset.x, offset.y - spacing.y)', () => {
        const spacing : Vector2 = new Vector2(Math.random() - 0.5 * 20 , Math.random() - 0.5 * 20);
        const offset : Vector2 = new Vector2(Math.random() - 0.5 * 200, Math.random() - 0.5 * 200);
        let axial : Vector2 = HexScreenToAxial(offset, spacing, offset);

        expect(axial.x).toBeCloseTo(0);
        expect(axial.y).toBeCloseTo(0);

        axial = HexScreenToAxial(new Vector2(offset.x, offset.y - spacing.y), spacing, offset);

        expect(axial.x).toBeCloseTo(0);
        expect(axial.y).toBeCloseTo(-1, 4);
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

