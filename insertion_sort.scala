object Main extends App {
  def isort(as: List[Int]): List[Int] = as match {
    case Nil   => Nil
    case x::xs => insert(x,isort(xs))
  }

  def insert(e: Int, as: List[Int]): List[Int] = as match {
    case Nil              => List(e)
    case x::_ if (e <= x) => e::as
    case x::xs            => x::insert(e,xs)
  }

  val mlist: List[Int] = List(4, 2, 8, 3, 9, 1)
  val s: List[Int] = isort(mlist)
  println(s"Before insertion sort: $mlist")
  println(s"After insertion sort: $s")
}
