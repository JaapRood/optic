import React, { useContext, useEffect, useState } from 'react';
import PropTypes from 'prop-types';
import isEqual from 'lodash.isequal';
import ButtonBase from '@material-ui/core/ButtonBase';
import TextField from '@material-ui/core/TextField';
import Typography from '@material-ui/core/Typography';
import { createStyles, makeStyles } from '@material-ui/styles';
import DoneIcon from '@material-ui/icons/Done';
import classNames from 'classnames';
import Button from '@material-ui/core/Button';
import debounce from 'lodash.debounce';
import Collapse from '@material-ui/core/Collapse';
import { LearnAPIPageContext } from './review-diff/learn-api/LearnAPIPageContext';
import { useHover } from '../utilities/useHoverHook';

export function urlStringToPathComponents(url) {
  const components = url.split('/').map((name, index) => {
    const isParameter = name.charAt(0) === ':' || name.charAt(0) === '{';
    return { index, name, originalName: name, isParameter };
  });
  const [root, ...rest] = components;
  if (root.name === '') {
    return rest;
  }
  return components;
}

const useStyles = makeStyles((theme) =>
  createStyles({
    thin: {
      fontWeight: 100,
    },
    component: {
      '&:hover': {
        color: '#cb5a77',
      },
      '&:focus': {
        color: '#cb5a77',
      },
    },
    thick: {
      fontWeight: 600,
    },
    paramLabel: {
      textAlign: 'left',
    },
  })
);

function PathComponentItem(props) {
  const classes = useStyles();
  const { item, updateItem, index, pathComponents } = props;
  const [hoverRef, isHovered] = useHover();

  function PathComponentButton({ onClick }) {
    return (
      <ButtonBase className={classes.component} onClick={onClick}>
        {item.isParameter ? (
          <Typography
            className={classNames(classes.thick, classes.paramLabel)}
          >{`{${item.name || '   '}}`}</Typography>
        ) : (
          <Typography className={classNames(classes.thin, classes.paramLabel)}>
            {item.name}
          </Typography>
        )}
      </ButtonBase>
    );
  }

  if (item.isParameter) {
    return (
      <PathComponentButton
        onClick={() =>
          updateItem({ ...item, name: item.originalName, isParameter: false })
        }
      />
    );
  }

  function ClickToMakeParam({ updateItem, classes, item }) {
    const { toDocument = [], pathExpressions = [] } = useContext(
      LearnAPIPageContext
    );

    function inferDefaultParamName() {
      if (pathComponents.length === 0) {
        return '';
      }

      const otherPathComponents = toDocument //qualifier, naive
        .map((doc) => {
          const expressions = pathExpressions[doc.id];
          return expressions.pathComponents;
        })
        .filter((i) => {
          const a = i
            .slice(0, index - 1)
            .map((c) => ({ name: c.name, isParameter: c.isParameter }));
          const b = pathComponents
            .slice(0, index - 1)
            .map((c) => ({ name: c.name, isParameter: c.isParameter }));

          return isEqual(a, b);
        });

      if (otherPathComponents.length === 0) {
        return '';
      } else {
        const firstMatchingParamName = otherPathComponents
          .map((i) => i.find((param) => param.index === index))
          .filter((param) => {
            if (param && param.isParameter) {
              return true;
            }
            return false;
          })[0];

        return firstMatchingParamName ? firstMatchingParamName.name : '';
      }
    }

    return (
      <PathComponentButton
        onClick={() =>
          updateItem({
            ...item,
            isParameter: true,
            name: inferDefaultParamName(),
          })
        }
      />
    );
  }

  return (
    <span ref={hoverRef}>
      {isHovered ? ( // when hovered bring in context aware variant
        <ClickToMakeParam
          item={item}
          classes={classes}
          updateItem={updateItem}
        />
      ) : (
        <PathComponentButton onClick={() => {}} /> // when not hovered, keep dumb
      )}
    </span>
  );
}

function UrlToPath(props) {
  const { url, onAccept, onUserCompleted } = props;
  const [pathComponents, setPathComponentsInternal] = useState(
    urlStringToPathComponents(url)
  );
  const [lastInteractedIndex, setLastInteractedIndex] = useState(null);
  const [collapseParams, setCollapseParams] = useState(false);
  const [firstTimeMap, setFirstTime] = useState({});

  function setPathComponents(pathComponents) {
    setPathComponentsInternal(pathComponents);
    onAccept(pathComponents);
  }

  useEffect(() => {
    onAccept(pathComponents);
  }, [onAccept]);

  function setItemAt(index) {
    return function (newItem) {

      const firstTime = !Boolean(firstTimeMap[index])

      setCollapseParams(firstTime ? newItem.name && newItem.isParameter : false)
      setLastInteractedIndex(index);
      setFirstTime((value) => ({...value, [index]: true}))

      setPathComponents(
        pathComponents.map((item) => {
          if (item.index === index) {
            return newItem;
          }
          return item;
        })
      );
    };
  }

  const parameters = pathComponents.filter((x) => x.isParameter);

  const canFinish =
    parameters.length === 0 || parameters.every((i) => Boolean(i.name));
  const finish = () => {
    setCollapseParams(true);
    onUserCompleted && onUserCompleted();
  };

  return (
    <div>
      <div style={{ display: 'flex', alignItems: 'center', flexWrap: 'wrap' }}>
        {pathComponents.flatMap((item) => [
          <ButtonBase key={`${item.index}-1`} disabled>
            <Typography
              style={{ marginRight: 2, marginLeft: 2, fontWeight: 200 }}
            >
              /
            </Typography>
          </ButtonBase>,
          <PathComponentItem
            key={`${item.index}-2`}
            item={item}
            index={item.index}
            pathComponents={pathComponents}
            updateItem={setItemAt(item.index)}
          />,
        ])}
      </div>
      {parameters.length > 0 ? (
        <Collapse in={!collapseParams}>
          <div>
            <br />
            <div>
              <Typography variant="overline">Parameter Names:</Typography>
            </div>
            {parameters.map((item, index) => {
              const updater = setItemAt(item.index);
              const changeValueOf = debounce((value) => {
                updater({ ...item, name: value });
              }, 500);
              return (
                <div
                  style={{ marginBottom: 11 }}
                  key={'name-param-' + index.toString()}
                >
                  <TextField
                    onKeyUp={(e) => {
                      if (e.keyCode === 13 && canFinish) finish();
                    }}
                    key={item.index}
                    autoFocus={item.index === lastInteractedIndex}
                    fullWidth
                    defaultValue={item.name}
                    label={`"${item.originalName}" is an example of a...`}
                    onChange={(e) => {
                      changeValueOf((e.target.value || '').replace(/\s/g, ''));
                    }}
                  >
                    {item.name}
                  </TextField>
                </div>
              );
            })}
            <Button
              startIcon={<DoneIcon />}
              disabled={!canFinish}
              color="primary"
              size="small"
              variant="contained"
              onClick={finish}
            >
              Done
            </Button>
          </div>
        </Collapse>
      ) : null}
    </div>
  );
}

UrlToPath.propTypes = {
  url: PropTypes.string.isRequired,
  onAccept: PropTypes.func.isRequired,
};

export default UrlToPath;
