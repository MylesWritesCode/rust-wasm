import type {
  OpenAPIClient,
  Parameters,
  UnknownParamsObject,
  OperationResponse,
  AxiosRequestConfig,
} from 'openapi-client-axios'; 

declare namespace Components {
    namespace Schemas {
        export interface GenerateDataResponse {
            edges: GraphEdge[];
            vertices: GraphVertex[];
        }
        export interface GraphEdge {
            /**
             * Random id, doesn't matter
             */
            id: string;
            /**
             * ID of the source vertex
             */
            source: string;
            /**
             * ID of the target vertex
             */
            target: string;
        }
        export interface GraphVertex {
            /**
             * Used to determine which edges are connected to this vertex
             */
            id: string;
            /**
             * A human-readable identifier for the vertex
             */
            label: string;
            /**
             * An optional parent vertex
             */
            parent: string;
        }
    }
}

export interface OperationMethods {
}

export interface PathsDictionary {
}

export type Client = OpenAPIClient<OperationMethods, PathsDictionary>
