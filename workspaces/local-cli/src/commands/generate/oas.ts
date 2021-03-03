import Command, { flags } from '@oclif/command';
import { getPathsRelativeToConfig } from '@useoptic/cli-config';
import { IPathMapping } from '@useoptic/cli-config';
import { OasProjectionHelper } from '@useoptic/domain';
import { cli } from 'cli-ux';
import fs from 'fs-extra';
import path from 'path';
import yaml from 'js-yaml';
import { developerDebugLogger, fromOptic } from '@useoptic/cli-shared';
import { lockFilePath } from '../../shared/paths';
import { Config } from '../../config';
import { Client, SpecServiceClient } from '@useoptic/cli-client';
import { getUser } from '../../shared/analytics';
import { EventEmitter } from 'events';
import { ensureDaemonStarted } from '@useoptic/cli-server';

export default class GenerateOas extends Command {
  static description = 'export an OpenAPI 3.0.1 spec';

  static flags = {
    json: flags.boolean({}),
    yaml: flags.boolean({}),
  };

  async run() {
    const { flags } = this.parse(GenerateOas);
    await generateOas(
      flags.yaml || (!flags.json && !flags.yaml) /* make this default */,
      flags.json
    );
    process.exit(0);
  }
}

export async function generateOas(
  flagYaml: boolean,
  flagJson: boolean
): Promise<{ json: string | undefined; yaml: string | undefined } | undefined> {
  try {
    const paths = await getPathsRelativeToConfig();
    const { specStorePath } = paths;
    try {
      const daemonState = await ensureDaemonStarted(
        lockFilePath,
        Config.apiBaseUrl
      );

      const apiBaseUrl = `http://localhost:${daemonState.port}/api`;
      developerDebugLogger(`api base url: ${apiBaseUrl}`);
      const cliClient = new Client(apiBaseUrl);
      cliClient.setIdentity(await getUser());
      const cliSession = await cliClient.findSession(paths.cwd, null, null);
      developerDebugLogger({ cliSession });

      const eventEmitter = new EventEmitter();
      const specService = new SpecServiceClient(
        cliSession.session.id,
        eventEmitter,
        apiBaseUrl
      );

      const eventsString = await specService.listEvents();
      cli.action.start('Generating OAS file');
      const parsedOas = OasProjectionHelper.fromEventString(eventsString);
      const outputFiles = await emit(paths, parsedOas, flagYaml, flagJson);
      const filePaths = Object.values(outputFiles);
      cli.action.stop(
        '\n' +
          fromOptic(
            `Generated OAS file${filePaths.length > 1 && 's'} \n` +
              filePaths.join('\n')
          )
      );

      return outputFiles;
    } catch (e) {
      console.error(e);
    }
  } catch (e) {
    console.error(e);
  }
}

export async function emit(
  paths: IPathMapping,
  parsedOas: object,
  flagYaml: boolean,
  flagJson: boolean
): Promise<{ json: string | undefined; yaml: string | undefined }> {
  const shouldOutputYaml = flagYaml;
  const shouldOutputJson = flagJson;

  const outputPath = path.join(paths.basePath, 'generated');

  let yamlPath, jsonPath;

  await fs.ensureDir(outputPath);
  if (shouldOutputYaml) {
    const outputFile = path.join(outputPath, 'openapi.yaml');
    await fs.writeFile(outputFile, yaml.safeDump(parsedOas, { indent: 1 }));
    yamlPath = outputFile;
  }
  if (shouldOutputJson) {
    const outputFile = path.join(outputPath, 'openapi.json');
    await fs.writeJson(outputFile, parsedOas, { spaces: 2 });
    jsonPath = outputFile;
  }

  return {
    json: jsonPath,
    yaml: yamlPath,
  };
}
