<?php
function getWidget($type)
{
    eval('$obj = new '.$type.'();');
    return $obj;

}//end getWidget()
