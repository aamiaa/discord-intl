const path = require('node:path');

const chokidar = require('chokidar');
const fg = require('fast-glob');
const debug = require('debug')('intl:metro-intl-transformer:watcher');

const { database } = require('./src/database');
const {
  IntlCompiledMessageFormat,
  isMessageDefinitionsFile,
} = require('@discord/intl-message-database');

const IGNORE_PATTERNS = [
  // Ignore our own compiled message files, even though they shouldn't have a matching extension.
  '*.compiled.messages.*',
  // Also ignore a bunch of default folders that just make globs/watches take forever.
  '**/node_modules/**',
  '**/target/**',
  '**/native/**',
  '**/dist/**',
  '**/build/**',
  '**/cache/**',
  '**/.cache/**',
  '**/__pycache__/**',
];
const MESSAGE_DEFINITION_FILE_PATTERNS = ['**/*.messages.js'];
const DEFAULT_LOCALE = 'en-US';

/**
 * @param {string} filePath
 */
function processFile(filePath) {
  if (!isMessageDefinitionsFile(filePath)) {
    return;
  }

  try {
    debug(`Processing file: ${filePath}`);
    // Convert the file name from `.messages.js` to `.compiled.messages.jsona` for output.
    const outputPath = filePath.replace(/\.messages\.js$/, '.compiled.messages.jsona');

    database.processDefinitionsFile(filePath);
    database.precompile(filePath, DEFAULT_LOCALE, outputPath, IntlCompiledMessageFormat.Json);
    debug(`Wrote definitions to: ${outputPath}`);
  } catch (e) {
    debug('[INTL Error] Failed to compile messages');
    console.error(e);
  }
}

/**
 * @param {string[]} watchedFolders
 * @param {{
 *  watch?: boolean
 * }} options
 */
async function compileIntlMessageFiles(watchedFolders, { watch = true } = {}) {
  const globs = watchedFolders.flatMap((folder) =>
    MESSAGE_DEFINITION_FILE_PATTERNS.map((pattern) => path.join(folder, pattern)),
  );
  debug(`Configured message file patterns:\n- ${globs.join('\n- ')}`);

  // Perform one initial scan and compilation to ensure all files exist before Metro might try to
  // resolve them.
  debug('Scanning for initial messages files');
  for await (const filePath of fg.stream(globs, {
    ignore: IGNORE_PATTERNS,
    absolute: true,
    onlyFiles: true,
  })) {
    processFile(filePath.toString());
  }
  debug('Initial message scan completed.');

  if (watch) {
    debug(`Setting up file watching for configured paths`);
    chokidar
      .watch(globs, { ignored: IGNORE_PATTERNS, ignoreInitial: true })
      .on('all', (event, filePath) => {
        debug(`Got event ${event} for ${filePath}`);
        processFile(filePath);
      });
  } else {
    debug('Not watching files because `watch` option was false');
  }
}

module.exports = { compileIntlMessageFiles };