/* eslint-disable */
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
};

export type Idea = {
  __typename?: 'Idea';
  /** Description of this idea. */
  desc: Scalars['String']['output'];
  /** Hash of this idea in its submitted form. */
  hash: Scalars['String']['output'];
  /**
   * Submitter of this idea's address. We track this for internal use sake, we don't actually
   * track if this was created, or if someone lied about the creation. We won't even display
   * this to the UI.
   */
  submitter: Scalars['String']['output'];
  /** Timestamp of this creation. */
  time: Scalars['Int']['output'];
};

export type Mutation = {
  __typename?: 'Mutation';
  explainIdea: Scalars['Boolean']['output'];
};


export type MutationExplainIdeaArgs = {
  desc: Scalars['String']['input'];
  submitter: Scalars['String']['input'];
};

export type Query = {
  __typename?: 'Query';
  ideas: Array<Idea>;
};

export type GetIdeasQueryVariables = Exact<{ [key: string]: never; }>;


export type GetIdeasQuery = { __typename?: 'Query', ideas: Array<{ __typename?: 'Idea', time: number, desc: string, hash: string }> };


export const GetIdeasDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"getIdeas"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"ideas"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"time"}},{"kind":"Field","name":{"kind":"Name","value":"desc"}},{"kind":"Field","name":{"kind":"Name","value":"hash"}}]}}]}}]} as unknown as DocumentNode<GetIdeasQuery, GetIdeasQueryVariables>;