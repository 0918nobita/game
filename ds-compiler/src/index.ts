import * as A from 'fp-ts/lib/Array';
import * as TE from 'fp-ts/lib/TaskEither';
import { pipe } from 'fp-ts/lib/pipeable';
import { TextLintEngine } from 'textlint';

pipe(
    TE.tryCatch(
        () => new TextLintEngine({ rules: ['no-hankaku-kana'] }).executeOnText('半角ｶﾀｶﾅ'),
        (reason) => console.error(reason),
    ),
    TE.map(results =>
        pipe(
            results,
            A.map(result => result.messages),
            A.flatten,
            A.reduce(
                '',
                (acc, msg) => `${acc}(Ln ${msg.line}, Col ${msg.column}) ${msg.message}\n`
            ),
            output => console.log(output)
        )
    ),
)();
