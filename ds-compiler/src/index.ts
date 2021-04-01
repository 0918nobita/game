import { TextlintMessage } from '@textlint/types';
import * as A from 'fp-ts/lib/Array';
import { flow, pipe } from 'fp-ts/lib/function';
import * as TE from 'fp-ts/lib/TaskEither';
import { TextLintEngine } from 'textlint';

// eslint-disable-next-line @typescript-eslint/no-var-requires
const { version } = require('../package.json') as { version: string };
console.log(`DramaScript Compiler v${version}`);

type LintTask = TE.TaskEither<string, TextlintMessage[]>;

const runLintTask = (task: LintTask) => task();

const lintText = (text: string): LintTask =>
    pipe(
        TE.tryCatch(() => {
            const rules = [
                'max-ten',
                'no-doubled-conjunction',
                'no-doubled-conjunctive-particle-ga',
                'no-hankaku-kana',
            ] as const;

            const engine = new TextLintEngine({ rules });

            return engine.executeOnText(text);
        }, String),
        TE.map(
            flow(
                A.map((result) => result.messages),
                A.flatten
            )
        )
    );

const showLintMessages: (task: LintTask) => LintTask = TE.map(
    A.map((msg) => {
        console.log(`(Ln ${msg.line}, Col ${msg.column}) ${msg.message}`);
        return msg;
    })
);

void pipe(
    lintText(
        'しかし、私は以下のように思ったが、結果は予想どおりだったが、これは禁止されている半角ｶﾀｶﾅ。しかし、同じ接続詞が連続して出現している。'
    ),
    showLintMessages,
    runLintTask
);
