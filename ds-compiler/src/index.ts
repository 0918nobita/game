import * as O from 'fp-ts/lib/Option';
import { pipe } from 'fp-ts/lib/pipeable';
import { TextLintEngine } from 'textlint';

const engine = new TextLintEngine({ rules: ['no-hankaku-kana'] });

(async (): Promise<void> => {
    const results = await engine.executeOnText('半角ｶﾀｶﾅ');

    for (const result of results)
        for (const msg of result.messages)
            console.log(`(Ln ${msg.line}, Col ${msg.column}) ${msg.message}`);
})();

pipe(
    O.bindTo('x')(O.some(10)),
    O.map(({ x }) => x + 1),
    O.chain(x => O.some(x + 2)),
    O.getOrElse(() => 0),
    x => console.log(x)
);  // => 13
