import React, { useEffect, useState } from 'react';
import Page from '../Page';
import createOpticDomain from '@useoptic/domain-rust';

export default function DocsPage() {
  const [domain, setDomain] = useState(null);

  useEffect(() => {
    createOpticDomain()
      .then((domain) => {
        setDomain(domain);
      })
      .catch((err) => {
        throw err;
      });
  }, []);

  if (!domain) return <div>Loading domain</div>;

  return (
    <Page>
      <Page.Navbar mini={true} />

      <Page.Body>{domain.greet('everyone')}</Page.Body>
    </Page>
  );
}
