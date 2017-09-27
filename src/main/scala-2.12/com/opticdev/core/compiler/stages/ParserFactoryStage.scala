package com.opticdev.core.compiler.stages

import com.opticdev.core.compiler.errors.AstPathNotFound
import com.opticdev.core.compiler.{FinderStageOutput, ParserFactoryOutput, SnippetStageOutput}
import com.opticdev.core.sdk.descriptions.Lens
import com.opticdev.core.sdk.descriptions.enums.Finders.FinderPath
import com.opticdev.core.sourcegear.accumulate.MapSchemaListener
import com.opticdev.core.sourcegear.gears.RuleProvider
import com.opticdev.core.sourcegear.gears.parsing.{NodeDesc, ParseAsModel}
import com.opticdev.parsers.AstGraph
import com.opticdev.parsers.graph.{AstPrimitiveNode, Child}
import com.opticdev.parsers.graph.path.FlatWalkablePath
import play.api.libs.json.JsObject

import scalax.collection.edge.LkDiEdge
import scalax.collection.mutable.Graph



class ParserFactoryStage(snippetStageOutput: SnippetStageOutput, finderStageOutput: FinderStageOutput)(implicit lens: Lens) extends CompilerStage[ParserFactoryOutput] {
  override def run: ParserFactoryOutput = {

    import com.opticdev.core.sdk.descriptions.helpers.ComponentImplicits._

    implicit val graph = snippetStageOutput.astGraph

    val enterOn = snippetStageOutput.entryChildren.head

    val nodeDescription = nodeToDescription(enterOn)

    val listeners = lens.components.schemaComponents.map(MapSchemaListener(_, lens.schema))

    implicit val ruleProvider = new RuleProvider()

    ParserFactoryOutput(
      ParseAsModel(
      nodeDescription,
      lens.schema,
      finderStageOutput.componentFinders.map {
        case (finderPath, components)=> (finderPathToFlatPath(finderPath, enterOn), components)
      },
      finderStageOutput.ruleFinders.map {
        case (finderPath, rules)=> (finderPathToFlatPath(finderPath, enterOn), rules)
      },
      listeners
    ))
  }

  def finderPathToFlatPath(finderPath: FinderPath, node: AstPrimitiveNode)(implicit graph: AstGraph): FlatWalkablePath = {
    val path = finderPath.fromNode(node)
    if (path.isEmpty) throw AstPathNotFound(finderPath)
    path.get.toFlatPath
  }

  def nodeToDescription(astPrimitiveNode: AstPrimitiveNode, edge: Child = Child(0, null)) : NodeDesc = {
    val children = astPrimitiveNode.children(snippetStageOutput.astGraph)
      .map(i=> nodeToDescription(i._2, i._1.asInstanceOf[Child]))

    import com.opticdev.core.sdk.PropertyValuesConversions._

    NodeDesc(
      astPrimitiveNode.nodeType,
      edge,
      astPrimitiveNode.properties.as[JsObject].toScala.value,
      children,
      Vector())
  }

}