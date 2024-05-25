// @ts-nocheck

/*
 * Un commentaire.
 */
// Single line
/* Multi
 * Line
 */

/**
 * Documentation.
 * 
 * @constructor
 * @param {number} x - Desc x.
 * @param {string} y - Desc y.
 * @returns {boolean}
 */

/// <reference lib="DOM" />

import exportParDefaut1 from "./import";
export { export1, export2 } from "./import";
import * as nom1 from "./import";
import { export1 } from "./import";
import { export1 as alias1 } from "./import";
import { export2, export3 } from "./import";
import { export4, export5 as alias2 } from "./import";
import exportParDefaut2, { export1 as alias3 } from "./import";
import exportParDefaut3, * as nom2 from "./import";
import "./import";

"Hello";
'World';
`Hello ${export1} !!`;

1;
2_000;
0x12;
7e12;
0.1;
.2;

// ------

let x1 = 1;
let x2 = export1 != export2;
let x3 = 0 > 1 && 0 < 1;

type safe<T> = NonNullable<T>;
type unsafe<T> = T | null | undefined;

interface Interface
{
    [Symbol.iterator](): Iterator<string>;
}

export enum OptionVariant {
    Some = "Some",
    None = "None",
}

class Option<T> {
    static None = <T = never>(): Option<T> => new this<T>(OptionVariant.None);

    static Some = <T>(value: T): Option<NonNullable<T>> => {
        if (value == null) {
            return None();
        }
        return new this(OptionVariant.Some, value);
    };

    static async fromAsync<T>(value: Promise<T>) {
        let x = await value;
        return Some(x);
    };

    constructor(
        public type: OptionVariant,
        private value?: unsafe<T>,
    ) { }

    expect(msg: string): safe<T> {
        if (this.is_some()) {
            return this.value as safe<T>;
        }
        throw new Error(`EXPECT: ${msg}`);
    }


    is_some(): this is Option<safe<T>> {
        return this.value != null;
    }

    filter(predicate: (value: safe<T>) => boolean): Option<T> {
        if (this.is_some()) {
            const value = this.unwrap();
            if (predicate(value)) {
                return Some(value);
            }
        }
        return None();
    }
}

const { Some, None } = Option;
export { Option, None, Some };

type Options = {
    /**
     * Remplace tout le reste d'une chaîne de caractères en minuscule.
     */
    to_lower?: boolean;
    /**
     * Inclure les séparateurs dans le résultat?
     */
    includes_separators?: boolean;
};

const INCLUDE_SEPARATOR: RegExp = /([\s-_:]+)/;
const EXCLUDE_SEPARATOR: RegExp = /[\s-_:]+/;

const Default: Options = {
    to_lower: true,
    includes_separators: true,
};

export function camelCase<T extends string>(
    text: T,
    user_options: Options = Default,
): Capitalize<T> {
    const options: Options = { ...Default, ...user_options };

    const algo = (word: string) => {
        if (word.length === 0) return word;
        type char = string[number];
        const first_ch: char = word[0].toUpperCase();
        const rest_of_str: string = word.slice(1);
        return first_ch + rest_of_str;
    };

    return algo(text) as Capitalize<T>;
}

async function get(url: string) {
    return fetch(url)
}
