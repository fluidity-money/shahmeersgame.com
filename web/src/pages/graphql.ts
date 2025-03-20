import { graphql } from "../gql";

export const ideasQuery = graphql(`
  query getIdeas{
    ideas {
      time
      desc
      hash
    }
  }
`);

export const explainMutation = graphql(`
  mutation explainIdea($desc: String!, $submitter: String!) {
    explainIdea(desc: $desc, submitter: $submitter)
  }
`);