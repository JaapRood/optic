package com.useoptic.contexts.shapes

import com.useoptic.contexts.shapes.projections.{ExampleProjection, TrailTags}
import com.useoptic.diff.{ChangeType, JsonFileFixture}
import com.useoptic.diff.shapes.JsonTrail
import com.useoptic.diff.shapes.JsonTrailPathComponent._
import org.scalatest.FunSpec
import io.circe.jawn.parse

class ExampleProjectionSpec extends FunSpec with JsonFileFixture {
  it("works for strings") {
    val json = parse("\"Hello\"").right.get
    val render = ExampleProjection.fromJson(json, "")
    assert(render.root.typeName.map(_.name).mkString(" ") == "\"Hello\"")
  }

  it("works for objects") {
    val basic = fromFile("basic-concept")
    val render = ExampleProjection.fromJson(basic, "")
    assert(render.root.fields.size == 3)
  }

  it("works for arrays") {
    val basic = fromFile("primitive-array")
    val render = ExampleProjection.fromJson(basic, "")
    assert(render.root.fields.size == 3)
  }

  describe("tagged fields from trails") {
    it("works for key in root object") {
      val basic = fromFile("basic-concept")
      val render = ExampleProjection.fromJson(basic, "", TrailTags(Map(
        JsonTrail(Seq(
          JsonObject(),
          JsonObjectKey("a")
        )) -> ChangeType.Addition
      )))

      assert(render.root.fields.find(_.fieldName == "a").get.tag.contains(ChangeType.Addition))
    }

    it("works for key in object, from array") {
      val basic = fromFile("todo-body")
      val render = ExampleProjection.fromJson(basic, "", TrailTags(Map(
        JsonTrail(Seq(
          JsonArray(),
          JsonArrayItem(0),
          JsonObject(),
          JsonObjectKey("task")
        )) -> ChangeType.Removal
      )))

      assert(render.root
        .fields.find(_.fieldName == "0").get.shape
        .fields.find(_.fieldName == "task").get.tag.contains(ChangeType.Removal))
    }

    it("works for nested key in object") {
      val basic = fromFile("nested")
      val render = ExampleProjection.fromJson(basic, "", TrailTags(Map(
        JsonTrail(Seq(
          JsonObject(),
          JsonObjectKey("nested"),
          JsonObject(),
          JsonObjectKey("nested2"),
          JsonArray(),
          JsonArrayItem(0)
        )) -> ChangeType.Addition
      )))

      assert(render.root
        .fields.find(_.fieldName == "nested").get.shape
        .fields.find(_.fieldName == "nested2").get.shape
        .fields.find(_.fieldName == "0").get.tag.contains(ChangeType.Addition))
    }
  }

}