import React from 'react';
import Drawer from '@material-ui/core/Drawer';
import CssBaseline from '@material-ui/core/CssBaseline';
import List from '@material-ui/core/List';
import Typography from '@material-ui/core/Typography';
import Divider from '@material-ui/core/Divider';
import ListItem from '@material-ui/core/ListItem';
import ListItemText from '@material-ui/core/ListItemText';
import makeStyles from '@material-ui/core/styles/makeStyles';
import Collapse from '@material-ui/core/Collapse';
import ExpandLess from '@material-ui/icons/ExpandLess';
import ExpandMore from '@material-ui/icons/ExpandMore';
import EndpointOverview from './EndpointOverview';
import {ListSubheader} from '@material-ui/core';
import {withRfcContext} from '../../contexts/RfcContext';
import {asPathTrail, isPathParameter} from '../../components/utilities/PathUtilities';
import ConceptOverview from './ConceptOverview';
import {DisplayPath} from './DisplayPath';
import {withNavigationContext} from '../../contexts/NavigationContext';
import compose from 'lodash.compose';
import {PathIdToPathString} from './PathIdToPathString';
import {updateContribution} from '../../engine/routines';
import { NavHashLink as NavLink } from 'react-router-hash-link';

const drawerWidth = 320;

const useStyles = makeStyles(theme => ({
  root: {
    display: 'flex',
  },
  appBar: {
    width: `calc(100% - ${drawerWidth}px)`,
    marginLeft: drawerWidth,
  },
  drawer: {
    width: drawerWidth,
    flexShrink: 0,
  },
  drawerPaper: {
    width: drawerWidth,
  },
  toolbar: {
    ...theme.mixins.toolbar,
    display: 'flex',
    flexDirection: 'row',
    justifyItems: 'center',
    alignItems: 'center',
  },
  apiName: {
    paddingLeft: 10,
    fontWeight: 500,
  },
  content: {
    flexGrow: 1,
    backgroundColor: theme.palette.background.default,
    padding: theme.spacing(3),
  },
  nested: {
    paddingLeft: 25,
    paddingTop: 5,
    overflow: 'hidden'
  },
  dense: {
    padding: 0,
    margin: 0,
  },
  subHeader: {
    backgroundColor: 'white'
  },
  sectionHeader: {
    padding: 20,
    paddingTop: 140
  }
}));

const EndpointBasePath = withRfcContext(withNavigationContext((props) => {
  const {path, operationsToRender, cachedQueryResults} = props;
  const classes = useStyles();

  const {contributions} = cachedQueryResults;
  const {name, children, depth, toggled, pathId, full, visible} = path;

  const url = full + name;


  const [open, setOpen] = React.useState(false);

  const handleClick = () => {
    setOpen(!open);
  };

  return (
    <>
      <ListItem button
                dense
                disableRipple
                onClick={handleClick}>
        <ListItemText primary={name.substr(1)}
                      classes={{dense: classes.dense}}
                      primaryTypographyProps={{variant: 'overline', style: {textTransform: 'none'}}}/>
        {open ? <ExpandLess/> : <ExpandMore/>}
      </ListItem>
      <Collapse in={open} timeout="auto" unmountOnExit>
        <List component="div"
              dense
              disablePadding>
          {operationsToRender.map(({requestId, request}) => {

            const {httpMethod, pathComponentId} = request.requestDescriptor;
            const purpose = contributions.getOrUndefined(requestId, 'purpose') || (
              <DisplayPath method={httpMethod} url={<PathIdToPathString pathId={pathComponentId}/>}/>
            );

            return (
              <NavLink
                to={`#${requestId}`}
                activeClassName="selected"
                style={{textDecoration: 'none', color: 'black'}}
              >
                <ListItem button
                          disableRipple
                          component="div"
                          dense
                          className={classes.nested}>
                  <ListItemText
                    primary={purpose}
                    classes={{dense: classes.dense}}
                    primaryTypographyProps={{
                      variant: 'overline',
                      style: {textTransform: 'none', textOverflow: 'ellipsis'}
                    }}/>
                </ListItem>
              </NavLink>
            );
          })}
        </List>
      </Collapse>
    </>
  );
}));


export default compose(withRfcContext, withNavigationContext)(function ApiOverview(props) {
  const {paths, concepts, cachedQueryResults, handleCommand} = props;
  const {notificationAreaComponent = null} = props;
  const classes = useStyles();


  function flatMapOperations(children) {
    return children.flatMap(path => {
      const requests = cachedQueryResults.requestIdsByPathId[path.pathId] || [];
      return requests.map(id => {
        return {
          requestId: id,
          request: cachedQueryResults.requests[id],
          path
        };
      }).concat(flatMapOperations(path.children));
    });
  }

  const operationsToRender = flatMapOperations(paths.children);

  return (
    <div className={classes.root}>
      <CssBaseline/>
      <Drawer
        className={classes.drawer}
        variant="permanent"
        classes={{
          paper: classes.drawerPaper,
        }}
        anchor="left"
      >
        <div className={classes.toolbar}>
          <Typography variant="subtitle1" className={classes.apiName}>{cachedQueryResults.apiName}</Typography>
          {/*<ApiSearch />*/}
        </div>
        <Divider/>
        <List
          component="nav"
          subheader={<ListSubheader className={classes.subHeader}>{'Endpoints'}</ListSubheader>}
          aria-labelledby="nested-list-subheader"
          dense={true}
        >
          {paths.children.map(i => <EndpointBasePath path={i}
                                                     operationsToRender={flatMapOperations([i])}/>)}
        </List>

        <Divider/>
        <List
          component="nav"
          subheader={<ListSubheader className={classes.subHeader}>{'Concepts'}</ListSubheader>}
          aria-labelledby="nested-list-subheader"
          dense={true}
        >
          {concepts.map(i => (
            <NavLink
              to={`#${i.shapeId}`}
              activeClassName="selected"
              style={{textDecoration: 'none', color: 'black'}}
            >
            <ListItem button dense disableRipple>
              <ListItemText
                primary={i.name}
                dense
                classes={{dense: classes.dense}}
                primaryTypographyProps={{variant: 'overline', style: {textTransform: 'none'}}}/>
            </ListItem>
            </NavLink>
          ))}
        </List>

      </Drawer>
      <main className={classes.content}>

        {notificationAreaComponent}

        <Typography variant="h3" color="primary" className={classes.sectionHeader}
                    style={{paddingTop: 20}}>Endpoints</Typography>

        {operationsToRender.map(operation => {
          const {pathsById, contributions} = cachedQueryResults;
          const pathTrail = asPathTrail(operation.path.pathId, pathsById);
          const pathParameters = pathTrail
            .map(pathId => pathsById[pathId])
            .filter((p) => isPathParameter(p))
            .map(p => ({
              pathId: p.pathId,
              name: p.descriptor.ParameterizedPathComponentDescriptor.name,
              description: contributions.getOrUndefined(p.pathId, 'description')
            }));

          return (
            <EndpointOverview
              endpointPurpose={contributions.getOrUndefined(operation.requestId, 'purpose')}
              endpointDescription={contributions.getOrUndefined(operation.requestId, 'description')}
              requestId={operation.requestId}
              method={operation.request.requestDescriptor.httpMethod}
              parameters={pathParameters}
              url={operation.path.full + operation.path.name}
              updateContribution={(id, key, value) => {
                handleCommand(updateContribution(id, key, value));
              }}
            />
          );
        })}

        <Typography variant="h3" color="primary" className={classes.sectionHeader}>Concepts</Typography>

        {concepts.map(concept => (
          <ConceptOverview
            name={concept.name}
            shapeId={concept.shapeId}
            example={{name: 'fizo', age: 15, breed: 'husky'}}
          />
        ))}

      </main>
    </div>
  );
});

