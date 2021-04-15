import { TextlintMessage } from '@textlint/types';
import * as A from 'fp-ts/lib/Array';
import * as E from 'fp-ts/lib/Either';
import { pipe } from 'fp-ts/lib/function';
import * as TE from 'fp-ts/lib/TaskEither';
import { TextLintCore } from 'textlint';

import { rules } from './rules';

export type LintTask = TE.TaskEither<string, TextlintMessage[]>;

export const runLintTask = (task: LintTask): Promise<E.Either<string, TextlintMessage[]>> => task();

export const lintText = (text: string): LintTask =>
    pipe(
        TE.tryCatch(() => {
            const core = new TextLintCore();
            core.setupRules(rules);
            return core.lintText(text);
        }, String),
        TE.map((result) => result.messages)
    );

export const showLintMessages: (task: LintTask) => LintTask = TE.map(
    A.map((msg) => {
        console.log(`(Ln ${msg.line}, Col ${msg.column}) ${msg.message}`);
        return msg;
    })
);
