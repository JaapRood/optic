package com.useoptic.diff.interactions.visitors

import com.useoptic.contexts.requests.Commands.{PathComponentId, RequestId, ResponseId}
import com.useoptic.coverage._
import com.useoptic.diff.interactions._
import com.useoptic.diff.shapes.ShapeTrail
import com.useoptic.dsa.Counter
import com.useoptic.types.capture.HttpInteraction

case class CoverageReport(coverageCounts: Counter[CoverageConcerns], diffs: Counter[InteractionDiffResult])

class CoveragePathVisitor(report: CoverageReport) extends PathVisitor {
  val diffVisitors = new DiffVisitors()

  override def visit(interaction: HttpInteraction, context: PathVisitorContext): Unit = {
    diffVisitors.pathVisitor.visit(interaction, context)
    val key = KeyFormatters.totalInteractions()
    report.coverageCounts.increment(key)
    if (context.path.isDefined) {
      val key = KeyFormatters.path(context.path.get)
      report.coverageCounts.increment(key)
    } else {
      val key = TotalUnmatchedPath()
      report.coverageCounts.increment(key)
    }
    diffVisitors.diffs.foreach(diff => {
      report.diffs.increment(diff)
    })
  }
}

class CoverageOperationVisitor extends OperationVisitor {
  override def begin(): Unit = ???

  override def visit(interaction: HttpInteraction, context: OperationVisitorContext): Unit = ???

  override def end(interaction: HttpInteraction, context: PathVisitorContext): Unit = ???
}

class CoverageRequestBodyVisitor(report: CoverageReport) extends RequestBodyVisitor {
  val diffVisitors = new DiffVisitors()

  override def begin(): Unit = {
    diffVisitors.requestBodyVisitor.begin()
  }

  override def visit(interaction: HttpInteraction, context: RequestBodyVisitorContext): Unit = {
    diffVisitors.requestBodyVisitor.visit(interaction, context)
  }

  override def end(interaction: HttpInteraction, context: PathVisitorContext): Unit = {
    diffVisitors.requestBodyVisitor.end(interaction, context)
    if (context.path.isEmpty) {
      return
    }
    if (diffVisitors.requestBodyVisitor.visitedWithMatchedContentTypes.nonEmpty) {
      val requestId = diffVisitors.requestBodyVisitor.visitedWithMatchedContentTypes.head

      val key = KeyFormatters.requestBody(requestId)
      report.coverageCounts.increment(key)

      report.coverageCounts.increment(TotalForPathAndMethod(context.path.get, interaction.request.method))

      report.coverageCounts.increment(
        interaction.request.body.contentType match {
          case Some(contentType) => TotalForPathAndMethodAndContentType(context.path.get, interaction.request.method, contentType)
          case None => TotalForPathAndMethodWithoutBody(context.path.get, interaction.request.method)
        }
      )

      if (diffVisitors.diffs.nonEmpty) {
        diffVisitors.diffs.foreach(diff => {
          report.diffs.increment(diff)
        })
      } else {
        diffVisitors.requestBodyVisitor.visitedShapeTrails.counts.foreach(entry => {
          val (shapeTrail, count) = entry

          val key = KeyFormatters.requestBodyShape(requestId, shapeTrail)
          report.coverageCounts.increment(key, count)
        })
      }
    }
  }
}

class CoverageResponseBodyVisitor(report: CoverageReport) extends ResponseBodyVisitor {
  val diffVisitors = new DiffVisitors()

  override def begin(): Unit = {
    diffVisitors.responseBodyVisitor.begin()
  }

  override def visit(interaction: HttpInteraction, context: ResponseBodyVisitorContext): Unit = {
    diffVisitors.responseBodyVisitor.visit(interaction, context)
  }

  override def end(interaction: HttpInteraction, context: PathVisitorContext): Unit = {
    diffVisitors.responseBodyVisitor.end(interaction, context)
    if (context.path.isEmpty) {
      return
    }
    if (diffVisitors.responseBodyVisitor.visitedWithMatchedContentTypes.nonEmpty) {
      val responseId = diffVisitors.responseBodyVisitor.visitedWithMatchedContentTypes.head

      val key = KeyFormatters.responseBody(responseId)
      report.coverageCounts.increment(key)

      report.coverageCounts.increment(TotalForPathAndMethodAndStatusCode(context.path.get, interaction.request.method, interaction.response.statusCode))

      report.coverageCounts.increment(
        interaction.response.body.contentType match {
          case Some(contentType) => TotalForPathAndMethodAndStatusCodeAndContentType(context.path.get, interaction.request.method, interaction.response.statusCode, contentType)
          case None => TotalForPathAndMethodAndStatusCodeWithoutBody(context.path.get, interaction.request.method, interaction.response.statusCode)
        }
      )

      if (diffVisitors.diffs.nonEmpty) {
        diffVisitors.diffs.foreach(diff => {
          report.diffs.increment(diff)
        })
      } else {
        diffVisitors.responseBodyVisitor.visitedShapeTrails.counts.foreach(entry => {
          val (shapeTrail, count) = entry
          val key = KeyFormatters.responseBodyShape(responseId, shapeTrail)
          report.coverageCounts.increment(key, count)
        })
      }
    }
  }
}

class CoverageVisitors extends Visitors {
  val coverageCounts = new Counter[CoverageConcerns]()
  val diffCounts = new Counter[InteractionDiffResult]()
  val report = CoverageReport(coverageCounts, diffCounts)

  override val pathVisitor: PathVisitor = new CoveragePathVisitor(report)
  override val operationVisitor: OperationVisitor = new CoverageOperationVisitor()
  override val requestBodyVisitor: RequestBodyVisitor = new CoverageRequestBodyVisitor(report)
  override val responseBodyVisitor: ResponseBodyVisitor = new CoverageResponseBodyVisitor(report)
}

object KeyFormatters {
  def totalInteractions(): CoverageConcerns = {
    TotalInteractions()
  }

  def path(pathId: PathComponentId): CoverageConcerns = {
    TotalForPath(pathId)
  }

  def request(pathId: PathComponentId, httpMethod: String): CoverageConcerns = {
    TotalForPathAndMethod(pathId, httpMethod)
  }

  def response(pathId: PathComponentId, httpMethod: String, httpStatusCode: Int): CoverageConcerns = {
    TotalForPathAndMethodAndStatusCode(pathId, httpMethod, httpStatusCode)
  }

  def requestBody(requestId: RequestId): CoverageConcerns = {
    TotalForRequest(requestId)
  }

  def responseBody(responseId: ResponseId): CoverageConcerns = {
    TotalForResponse(responseId)
  }

  def requestBodyShape(requestId: RequestId, shapeTrail: ShapeTrail): CoverageConcerns = {
    TotalForRequestBodyItem(requestId, shapeTrail)
  }

  def responseBodyShape(responseId: ResponseId, shapeTrail: ShapeTrail): CoverageConcerns = {
    TotalForResponseBodyItem(responseId, shapeTrail)
  }
}