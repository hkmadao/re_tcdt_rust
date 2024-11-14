import React, { FC, useState, useEffect, } from 'react';
import { TLayout } from '@/models';
import { layoutConf, } from '../conf';
import { useAsso, } from '../hooks';

const Layout: FC<
  Omit<TLayout, 'children'> & { layoutChildren: TLayout[]; children: any }
> = ({
  id,
  direction,
  flexStr,
  children,
  ...props
}) => {
    const asso = useAsso(id, layoutConf);
    return (
      <>
        <div
          id={id}
{%- raw %}
          style={{
            display: 'flex',
            flexDirection: direction,
            margin: '5px',
            flex: flexStr ?? '1 1 auto',
            overflow: 'auto',
            // cursor: (asso && asso.disabled) ? 'not-allowed' : undefined,
            // pointerEvents: (asso && asso.disabled) ? 'none' : undefined,
            // opacity: (asso && asso.disabled) ? '0.5' : undefined,
          }}
{%- endraw %}
        >
          {children}
        </div>
      </>
    );
  };

export default Layout;
