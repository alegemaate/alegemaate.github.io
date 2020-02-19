import styled from "styled-components";

export const Wrapper = styled.div`
  display: grid;
  grid-template-columns: 1fr 1fr;
  grid-column-gap: 36px;

  @media (max-width: 800px) {
    grid-template-columns: 1fr;
  }

  @media (max-width: 500px) {
    grid-template-columns: 1fr;
  }
`;

export const Box = styled.div`
  border-top: 3px solid #0f0a01;
  ${props => (props.center ? "display: flex; align-items: center;" : "")}
`;

export const Spacer = styled.div`
  width: 100%;
  display: block;
  height: 0.8em;
`;
