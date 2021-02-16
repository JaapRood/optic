export function debugDump(specService, captureId) {
  return async function (sanitizeBodies = true) {
    const events = await specService.listEvents();
    const session = await specService.listCapturedSamples(captureId);
    const sessionCleaned = JSON.parse(JSON.stringify(session));

    console.log('sanitizing data...' + sanitizeBodies);
    sessionCleaned.samples.forEach((i) => {
      if (sanitizeBodies) {
        i.request.body.value.asJsonString = null;
        i.request.body.value.asText = null;

        i.response.body.value.asJsonString = null;
        i.response.body.value.asText = null;
      } else {
        i.request.body.value.asShapeHashBytes = null;
        i.response.body.value.asShapeHashBytes = null;
      }
    });

    const output = JSON.stringify(
      {
        events: JSON.parse(events),
        session: sessionCleaned,
        examples: {},
      },
      null,
      2
    );

    const blob = new Blob([output], { type: 'application/json' });
    const url = window.URL.createObjectURL(blob);

    const link = document.createElement('a');
    link.href = url;
    link.download = `debug-capture-${captureId}-${Math.floor(
      Date.now() / 1000
    )}.json`;
    console.log(link);
    link.click();
  };
}
