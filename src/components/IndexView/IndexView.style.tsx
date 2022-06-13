import styled from "styled-components";

export const Wrapper = styled.div`
  display: grid;
  grid-template-columns: 1fr 1fr;

  @media (max-width: 800px) {
    grid-template-columns: 1fr;
  }

  @media (max-width: 500px) {
    grid-template-columns: 1fr;
  }
`;

export const Box = styled.div<{ center?: boolean; noPad?: boolean }>`
  border-top: 10px solid #0f0a01;
  ${(props): string =>
    props.center ?? false ? "display: flex; align-items: center;" : ""}
  ${(props): string => (props.noPad ?? false ? "" : "padding: 24px;")}
`;

export const Spacer = styled.div`
  width: 100%;
  display: block;
  height: 0.8em;
`;
