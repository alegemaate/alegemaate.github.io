import styled from "styled-components";

export const LinkStyle = styled.div`
  font-family: "Akzidenz Grotesk", "Helvetica Neue", "Helvetica", sans-serif;
  background-color: #eaded0;
  display: flex;
  padding: 20px;
  align-items: center;
  cursor: pointer;

  &:hover {
    font-family: "Akzidenz Grotesk Italic", "Helvetica Neue", "Helvetica",
      sans-serif;
    background-color: #b7c7bd;
  }
`;

export const LinkText = styled.div`
  width: 80%;
  display: flex;
  align-items: center;
  justify-content: flex-start;
`;

export const LinkIcon = styled.div`
  width: 20%;
  display: flex;
  align-items: center;
  justify-content: flex-end;
  font-size: 1.4em;
`;
