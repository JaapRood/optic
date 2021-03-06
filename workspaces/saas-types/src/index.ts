////////////////////////////////////////////////////////////////////////////////
export type ISO8601String = string;

export interface ILink {
  rel: string;
  href: string;
}

////////////////////////////////////////////////////////////////////////////////

export interface IUploadLocationSpecification {
  type: 's3';
  value: string;
}

export interface ICaptureMetadata {
  deploymentId: string;
  buildId: string;
  environmentName: string;
  apiName: string | null;
}

export interface ICreateCaptureRequest {
  specLocation: IUploadLocationSpecification;
  opticConfig: {
    ignoreRequests?: string[];
  };
  captureMetadata: ICaptureMetadata;
}

export interface ICreateCaptureResponse {
  capture: {
    id: string;
  };
  agentToken: string;
}

////////////////////////////////////////////////////////////////////////////////
export type CaptureId = string;
export type OrgId = string;
export type AgentGroupId = string;
export type ApiId = string;

export interface ICaptureListResponseBody {
  captures: ICaptureResponseBody[];
}

////////////////////////////////////////////////////////////////////////////////

export interface ITag {
  name: string;
  value: string;
}

export interface ICaptureResponseBody {
  captureId: CaptureId;
  createdAt: ISO8601String;
  updatedAt: ISO8601String;
  completedAt: ISO8601String | null;
  tags: Array<ITag>;
  links: ILink[];
}

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
