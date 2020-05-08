import React, { useEffect, useState } from 'react';
import Page from '../Page';
import createOpticDomain from '@useoptic/domain-rust';
import { useMockData } from '../../contexts/MockDataContext';

export default function DocsPage() {
  const [domain, setDomain] = useState(null);
  const { available, data, loading, error } = useMockData();

  useEffect(() => {
    createOpticDomain()
      .then((domain) => {
        setDomain(domain);
      })
      .catch((err) => {
        throw err;
      });
  }, []);

  if (error) throw error;

  if (!available)
    return (
      <div>
        Docs powered by Rust domain currently only works against exapmle session
      </div>
    );

  if (!domain || loading) return <div>Loading domain</div>;

  return (
    <Page>
      <Page.Navbar mini={true} />
      <Endpoints domain={domain} events={data.events} />
      <Page.Body></Page.Body>
    </Page>
  );
}

function Endpoints({ domain, events }) {
  const rfcState = domain.rfc_state_from_events(events);

  console.log(rfcState);

  return <div>Render endpoints</div>;
}
