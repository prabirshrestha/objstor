import { Link as ChakraUiLink, LinkProps as ChakraUiLinkProps} from '@chakra-ui/react';
import { Link as ReactRouterLink } from 'react-router-dom';

export const Link = (props: ChakraUiLinkProps & { to: string }) => {
  return (
    <ChakraUiLink as={ReactRouterLink} to={props.to}>
      {props.children}
    </ChakraUiLink>
  );
};
