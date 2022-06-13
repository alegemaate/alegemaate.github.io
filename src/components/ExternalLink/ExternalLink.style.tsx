import styled from "styled-components";

export const LinkStyle = styled.a`
  font-family: "Akzidenz Grotesk", "Helvetica Neue", "Helvetica", sans-serif;
  background-color: #ffeccc;
  display: flex;
  padding: 20px;
  align-items: center;
  cursor: pointer;
  transition: all 0.2s ease-in-out;
  border-radius: 5px;

  &:hover {
    font-family: "Akzidenz Grotesk Italic", "Helvetica Neue", "Helvetica",
      sans-serif;
    background-color: #514538;
    color: #ffffff;
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
