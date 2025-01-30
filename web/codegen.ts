import type { CodegenConfig } from '@graphql-codegen/cli'

const config: CodegenConfig = {
  schema: "/home/user/Documents/fluidity/shahmeersgame.com/cmd/graphql.webapp/graph/schema.graphqls",
  documents: ['src/**/*.tsx'],
  ignoreNoDocuments: true, // for better experience with the watcher
  generates: {
    './src/gql/': {
      preset: 'client'
    }
  }
}

export default config
