import { build } from 'esbuild';
import path from 'path';

async function main(): Promise<void> {
    try {
        await build({
            platform: 'node',
            entryPoints: [path.join(__dirname, '../src/index.ts')],
            outfile: path.join(__dirname, '../dist/bundle.js'),
            bundle: true,
            external: [
                'textlint',
                'textlint-rule-max-ten',
                'textlint-rule-no-doubled-conjunction',
                'textlint-rule-no-doubled-conjunctive-particle-ga',
                'textlint-rule-no-hankaku-kana',
            ],
            minify: true,
            sourcemap: true,
        });
    } catch (e) {
        console.error('Failed to build while executing esbuild');
        console.error(e);
    }
}

void main();
